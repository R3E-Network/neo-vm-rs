use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::exception_handling_context::ExceptionHandlingContext;

use super::{
	evaluation_stack::EvaluationStack,
	instruction::Instruction,
	reference_counter::ReferenceCounter,
	script::{Script, ScriptError},
	slot::Slot,
};

pub struct SharedStates {
	pub script: Rc<RefCell<Script>>,
	pub evaluation_stack: Rc<RefCell<EvaluationStack>>,
	pub static_fields: Option<Rc<RefCell<Slot>>>,
	pub states: HashMap<String, Box<dyn std::any::Any>>,
}

pub struct ExecutionContext {
	pub shared_states: Rc<RefCell<SharedStates>>,
	pub rv_count: i32,
	pub instruction_pointer: usize,
	pub local_variables: Option<Rc<RefCell<Slot>>>,
	pub arguments: Option<Rc<RefCell<Slot>>>,
	pub try_stack: Option<Vec<ExceptionHandlingContext>>,
}

impl ExecutionContext {
	pub fn new(
		script: Rc<RefCell<Script>>,	
		rv_count: i32,
		reference_counter: Rc<RefCell<ReferenceCounter>>,
	) -> Self {
		let shared_states = Rc::new(RefCell::new(SharedStates {
			script: Rc::clone(&script),
			evaluation_stack: Rc::new(RefCell::new(EvaluationStack::new(reference_counter))),
			static_fields: None,
			states: HashMap::new(),
		}));

		ExecutionContext {
			shared_states,
			rv_count,
			instruction_pointer: 0,
			local_variables: None,
			arguments: None,
			try_stack: None,
		}
	}

	pub fn rv_count(&self) -> i32 {
		self.rv_count
	}

	pub fn script(&self) -> Rc<RefCell<Script>> {
		Rc::clone(&self.shared_states.borrow_mut().script)
	}
	pub fn evaluation_stack(&self) -> Rc<RefCell<EvaluationStack>> {
		Rc::clone(&self.shared_states.borrow_mut().evaluation_stack)
	}

	pub fn static_fields(&self) -> Option<Rc<RefCell<Slot>>> {
		self.shared_states.borrow_mut().static_fields.clone()
	}

	pub fn set_static_fields(&mut self, value: Option<Rc<RefCell<Slot>>>) {
		self.shared_states.borrow_mut().static_fields = value;
	}

	pub fn local_variables(&self) -> Option<Rc<RefCell<Slot>>> {
		self.local_variables.clone()
	}

	pub fn set_local_variables(&mut self, value: Option<Rc<RefCell<Slot>>>) {
		self.local_variables = value;
	}

	pub fn arguments(&self) -> Option<Rc<RefCell<Slot>>> {
		self.arguments.clone()
	}

	pub fn set_arguments(&mut self, value: Option<Rc<RefCell<Slot>>>) {
		self.arguments = value;
	}

	pub fn try_stack(&self) -> Option<&Vec<ExceptionHandlingContext>> {
		self.try_stack.as_ref()
	}

	pub fn try_stack_mut(&mut self) -> Option<&mut Vec<ExceptionHandlingContext>> {
		self.try_stack.as_mut()
	}

	pub fn instruction_pointer(&self) -> usize {
		self.instruction_pointer
	}

	pub fn set_instruction_pointer(&mut self, value: usize) -> Result<(), &'static str> {
		if value > self.script().borrow_mut().len() {
			return Err("Instruction pointer out of range");
		}
		self.instruction_pointer = value;
		Ok(())
	}

	pub fn current_instruction(&self) -> Option<Instruction> {
		self.get_instruction(self.instruction_pointer)
	}

	pub fn next_instruction(&self) -> Option<Instruction> {
		self.current_instruction()
			.and_then(|current| self.get_instruction(self.instruction_pointer + current.size()))
	}

	pub fn clone(&self) -> Self {
		self.clone_with_ip(self.instruction_pointer)
	}

	pub fn clone_with_ip(&self, initial_position: usize) -> Self {
		ExecutionContext {
			shared_states: Rc::clone(&self.shared_states),
			rv_count: 0,
			instruction_pointer: initial_position,
			local_variables: None,
			arguments: None,
			try_stack: None,
		}
	}

	fn get_instruction(&self, ip: usize) -> Result<Rc<Instruction>, ScriptError> {
		if ip >= self.script().borrow_mut().len() {
			Err(ScriptError::InvalidInstructionPointer(ip))
		} else {
			self.script().borrow_mut().get_instruction(ip)
		}
	}

	pub fn get_state<T: 'static>(&mut self, factory: Option<Box<dyn Fn() -> T>>) -> Rc<RefCell<T>> {
		let type_name = std::any::type_name::<T>();
		let states = &mut self.shared_states.borrow_mut().states;

		if !states.contains_key(type_name) {
			let value: T = match factory {
				Some(f) => f(),
				None => T::default(),
			};
			states.insert(type_name.to_string(), Box::new(RefCell::new(value)));
		}

		let state = states.get(type_name).unwrap();
		Rc::new(RefCell::new(state.downcast_ref::<T>().unwrap().clone()))
	}

	pub fn move_next(&mut self) -> bool {
		if let Some(current) = self.current_instruction() {
			self.instruction_pointer += current.size();
			self.instruction_pointer < self.script().borrow_mut().len()
		} else {
			false
		}
	}
}
