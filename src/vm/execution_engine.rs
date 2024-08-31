use crate::{
	evaluation_stack::EvaluationStack,
	execution_context::ExecutionContext,
	instruction::Instruction,
	jump_table::JumpTable,
	types::stack_item::StackItem,
	vm::{
		execution_engine_limits::ExecutionEngineLimits, reference_counter::ReferenceCounter,
		script::Script, vm_error::VMError,
	},
	vm_state::VMState,
};
use std::{cell::RefCell, rc::Rc};

/// Represents the VM used to execute the script.
pub struct ExecutionEngine {
	pub state: VMState,
	pub is_jumping: bool,
	pub jump_table: Rc<JumpTable>,
	pub limits: ExecutionEngineLimits,
	pub reference_counter: Rc<RefCell<ReferenceCounter>>,
	pub invocation_stack: Vec<Rc<RefCell<ExecutionContext>>>,
	pub current_context: Option<Rc<RefCell<ExecutionContext>>>,
	pub entry_context: Option<Rc<RefCell<ExecutionContext>>>,
	pub result_stack: Rc<RefCell<EvaluationStack>>,
	pub uncaught_exception: Option<Rc<RefCell<StackItem>>>,
}

impl ExecutionEngine {
	pub fn is_jumping(&self) -> bool {
		self.is_jumping
	}

	pub fn set_is_jumping(&mut self, is_jumping: bool) {
		self.is_jumping = is_jumping;
	}

	pub fn jump_table(&self) -> &Rc<JumpTable> {
		&self.jump_table
	}

	pub fn set_jump_table(&mut self, jump_table: Rc<JumpTable>) {
		self.jump_table = jump_table;
	}

	pub fn limits(&self) -> &ExecutionEngineLimits {
		&self.limits
	}

	pub fn set_limits(&mut self, limits: ExecutionEngineLimits) {
		self.limits = limits;
	}

	pub fn reference_counter(&self) -> &Rc<RefCell<ReferenceCounter>> {
		&self.reference_counter
	}

	pub fn set_reference_counter(&mut self, reference_counter: Rc<RefCell<ReferenceCounter>>) {
		self.reference_counter = reference_counter;
	}

	pub fn invocation_stack(&self) -> &Vec<Rc<RefCell<ExecutionContext>>> {
		&self.invocation_stack
	}

	pub fn set_invocation_stack(&mut self, invocation_stack: Vec<Rc<RefCell<ExecutionContext>>>) {
		self.invocation_stack = invocation_stack;
	}

	pub fn current_context(&self) -> &Option<Rc<RefCell<ExecutionContext>>> {
		&self.current_context
	}

	pub fn set_current_context(&mut self, current_context: Option<Rc<RefCell<ExecutionContext>>>) {
		self.current_context = current_context;
	}

	pub fn entry_context(&self) -> &Option<Rc<RefCell<ExecutionContext>>> {
		&self.entry_context
	}

	pub fn set_entry_context(&mut self, entry_context: Option<Rc<RefCell<ExecutionContext>>>) {
		self.entry_context = entry_context;
	}

	pub fn result_stack(&self) -> &Rc<RefCell<EvaluationStack>> {
		&self.result_stack
	}

	pub fn set_result_stack(&mut self, result_stack: Rc<RefCell<EvaluationStack>>) {
		self.result_stack = result_stack;
	}

	pub fn uncaught_exception(&self) -> &Option<Rc<RefCell<StackItem>>> {
		&self.uncaught_exception
	}

	pub fn set_uncaught_exception(&mut self, uncaught_exception: Option<Rc<RefCell<StackItem>>>) {
		self.uncaught_exception = uncaught_exception;
	}
}

impl ExecutionEngine {
	pub fn new(jump_table: Option<Rc<JumpTable>>) -> Self {
		Self::new_with_limits(
			jump_table,
			Rc::new(RefCell::new(ReferenceCounter::new())),
			ExecutionEngineLimits::default(),
		)
	}

	pub fn new_with_limits(
		jump_table: Option<Rc<JumpTable>>,
		reference_counter: Rc<RefCell<ReferenceCounter>>,
		limits: ExecutionEngineLimits,
	) -> Self {
		ExecutionEngine {
			state: VMState::Break,
			is_jumping: false,
			jump_table: jump_table.unwrap_or_else(|| Rc::new(JumpTable::default())),
			limits,
			reference_counter: Rc::clone(&reference_counter),
			invocation_stack: Vec::new(),
			current_context: None,
			entry_context: None,
			result_stack: Rc::new(RefCell::new(EvaluationStack::new(reference_counter))),
			uncaught_exception: None,
		}
	}

	pub fn execute(&mut self) -> VMState {
		if self.state == VMState::Break {
			self.state = VMState::None;
		}
		while self.state != VMState::Halt && self.state != VMState::Fault {
			self.execute_next();
		}
		self.state
	}

	pub fn execute_next(&mut self) {
		if self.invocation_stack.is_empty() {
			self.state = VMState::Halt;
		} else {
			match self.execute_instruction() {
				Ok(_) => {},
				Err(e) => self.on_fault(&e),
			}
		}
	}

	fn execute_instruction(&mut self) -> Result<(), VMError> {
		let context = self
			.current_context
			.as_ref()
			.ok_or(VMError::Custom("No current context".to_string()))?;
		let instruction = context
			.borrow()
			.current_instruction()
			.ok_or(VMError::Custom("No current instruction".to_string()))?;
		self.pre_execute_instruction(&instruction);

		// Execute the instruction
		self.jump_table.execute(self, &instruction);
		self.post_execute_instruction(&instruction)?;
		if !self.is_jumping {
			context.borrow_mut().move_next();
		}
		self.is_jumping = false;

		Ok(())
	}

	pub fn load_script(
		&mut self,
		script: Rc<RefCell<Script>>,
		rv_count: i32,
		initial_position: usize,
	) -> Result<Rc<RefCell<ExecutionContext>>, VMError> {
		let context = self.create_context(script, rv_count, initial_position)?;
		self.load_context(context.clone())?;
		Ok(context)
	}

	fn create_context(
		&self,
		script: Rc<RefCell<Script>>,
		rv_count: i32,
		initial_position: usize,
	) -> Result<Rc<RefCell<ExecutionContext>>, VMError> {
		if rv_count < -1 || rv_count > u16::MAX as i32 {
			return Err(VMError::InvalidParameter("Invalid rv_count".to_string()));
		}
		let context = Rc::new(RefCell::new(ExecutionContext::new(
			Rc::clone(&script),
			rv_count,
			Rc::clone(&self.reference_counter),
		)));
		context.borrow_mut().set_instruction_pointer(initial_position)?;
		Ok(context)
	}

	pub fn load_context(&mut self, context: Rc<RefCell<ExecutionContext>>) -> Result<(), VMError> {
		if self.invocation_stack.len() >= self.limits.max_invocation_stack_size {
			return Err(VMError::InvocationStackOverflow(
				"MaxInvocationStackSize exceeded".to_string(),
			));
		}
		self.invocation_stack.push(Rc::clone(&context));
		if self.entry_context.is_none() {
			self.entry_context = Some(Rc::clone(&context));
		}
		self.current_context = Some(context);
		Ok(())
	}

	pub fn unload_context(&mut self, context: Rc<RefCell<ExecutionContext>>) {
		if let Some(last) = self.invocation_stack.last() {
			if !Rc::ptr_eq(last, &context) {
				return;
			}
		}
		self.invocation_stack.pop();

		if self.invocation_stack.is_empty() {
			self.current_context = None;
			self.entry_context = None;
		} else {
			self.current_context = self.invocation_stack.last().cloned();
		}

		let mut context = context.borrow_mut();
		if let Some(static_fields) = context.static_fields() {
			if self.current_context.as_ref().map_or(true, |current| {
				current.borrow().static_fields() != Some(Rc::clone(&static_fields))
			}) {
				static_fields.borrow_mut().clear_references();
			}
		}
		if let Some(local_variables) = context.local_variables() {
			local_variables.borrow_mut().clear_references();
		}
		if let Some(arguments) = context.arguments() {
			arguments.borrow_mut().clear_references();
		}
	}

	fn on_fault(&mut self, error: &VMError) {
		self.state = VMState::Fault;
		// Additional fault handling logic can be added here
	}

	fn pre_execute_instruction(&mut self, instruction: &Instruction) {
		// Pre-execution logic can be added here
	}

	fn post_execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VMError> {
		if self.reference_counter.borrow().count() < self.limits.max_stack_size {
			return Ok(());
		}
		if self.reference_counter.borrow().check_zero_referred() > self.limits.max_stack_size {
			return Err(VMError::StackOverflow("MaxStackSize exceeded".to_string()));
		}
		Ok(())
	}

	pub fn peek(&self, index: usize) -> Result<Rc<RefCell<StackItem>>, VMError> {
		let context = self
			.current_context
			.as_ref()
			.ok_or(VMError::Custom("No current context".to_string()))?;
		context.borrow().evaluation_stack().borrow().peek(index)
	}

	pub fn pop(&mut self) -> Result<Rc<RefCell<StackItem>>, VMError> {
		let context = self
			.current_context
			.as_ref()
			.ok_or(VMError::Custom("No current context".to_string()))?;
		context.borrow_mut().evaluation_stack()
	}

	pub fn push(&mut self, item: Rc<RefCell<StackItem>>) {
		let context = self
			.current_context
			.as_ref()
			.ok_or(VMError::Custom("No current context".to_string()))?;
		context.borrow_mut().evaluation_stack().borrow_mut().push(item);
	}

	pub fn state(&self) -> VMState {
		self.state
	}

	pub fn set_state(&mut self, new_state: VMState) {
		if self.state != new_state {
			self.state = new_state;
			self.on_state_changed();
		}
	}

	fn on_state_changed(&mut self) {
		// State change handling logic can be added here
	}
}

impl Drop for ExecutionEngine {
	fn drop(&mut self) {
		self.invocation_stack.clear();
	}
}
