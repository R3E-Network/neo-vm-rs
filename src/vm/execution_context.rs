use crate::{
	evaluation_stack::EvaluationStack,
	exception::exception_handling_context::ExceptionHandlingContext,
};
use std::{
	any::{Any, TypeId},
	cell::{Ref, RefCell},
	collections::HashMap,
	rc::Rc,
};
use crate::instruction::Instruction;
use crate::slot::Slot;
use crate::vm::reference_counter::ReferenceCounter;
use crate::vm::script::Script;

pub struct ExecutionContext {
	pub shared_states: Rc<RefCell<SharedStates>>,
	pub instruction_pointer: usize,

	/// The number of return values when this context returns.
	pub rv_count: i32,

	/// The local variables of this context.
	pub local_variables: Option<Slot>,

	/// The arguments passed to this context.
	pub arguments: Option<Slot>,

	/// The try stack to handle exceptions.
	pub try_stack: Option<Vec<ExceptionHandlingContext>>,
}

pub struct SharedStates {
	pub(crate) script: Script,
	pub(crate) evaluation_stack: Rc<RefCell<EvaluationStack>>,
	pub(crate) static_fields: Option<Slot>,
	pub(crate) states: HashMap<TypeId, Box<dyn Any>>,
}

impl ExecutionContext {
	pub fn new(script: Script, reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		let shared_states = SharedStates {
			script,
			evaluation_stack: Ref::new(RefCell::new(EvaluationStack::new(reference_counter))),
			static_fields: None,
			states: HashMap::new(),
		};
		Self {
			shared_states:Rc::new(RefCell::new(shared_states)),
			instruction_pointer: 0,
			rv_count: 0,
			local_variables: None,
			arguments: None,
			try_stack: None,
		}
	}

	// Other fields and methods

	pub fn get_state<T: 'static>(&mut self) -> &mut T
	where
		T: Default + Any,
	{
		self.shared_states
			.borrow()
			.states
			.entry(TypeId::of::<T>())
			.or_insert_with(|| Box::new(Default::default()))
			.downcast_mut::<T>()
			.unwrap()
	}

	// pub fn peek(&self, index: usize) -> Rc<RefCell<VMStackItem>> {
	// 	let idx = self.items.len() - index - 1;
	// 	&self.items[idx]
	// }

	// pub fn push(&mut self, item:Rc<RefCell<VMStackItem>>) {
	// 	self.items.push(item);
	// 	self.reference_counter.add_stack_reference(&item);
	// }
	//
	// pub fn pop(&mut self) -> Rc<RefCell<VMStackItem>> {
	// 	let item = self.items.pop().expect("stack empty");
	// 	self.reference_counter.remove_stack_reference(&item);
	// 	item
	// }
	//
	// pub fn remove(&mut self, index: usize) -> Rc<RefCell<VMStackItem>> {
	// 	let idx = self.items.len() - index - 1;
	// 	let item = self.items.remove(idx).expect("index out of bounds");
	// 	self.reference_counter.remove_stack_reference(&item);
	// 	item.try_into().unwrap()
	// }

	pub fn evaluation_stack(&self) -> Rc<RefCell<EvaluationStack>> {
		self.shared_states.borrow().evaluation_stack.clone()
	}
	pub fn evaluation_stack_mut(&mut self) -> Rc<RefCell<EvaluationStack>> {
		self.shared_states.borrow().evaluation_stack.clone()
	}

	pub fn script(&self) -> &Script {
		&self.shared_states.borrow().script
	}
	pub fn script_mut(&mut self) -> &mut Script {
		&mut self.shared_states.borrow().script
	}

	pub fn fields(&self) -> Option<&Slot> {
		self.shared_states.borrow().static_fields.as_ref()
	}

	pub fn fields_mut(&mut self) -> Option<&mut Slot> {
		self.shared_states.borrow().static_fields.as_mut()
	}
	pub fn states(&self) -> &HashMap<TypeId, Box<dyn Any>> {
		&self.shared_states.borrow().states
	}
	pub fn states_mut(&mut self) -> &mut HashMap<TypeId, Box<dyn Any>> {
		&mut self.shared_states.borrow().states
	}

	pub fn move_next(&mut self) {
		self.instruction_pointer += 1;

		if self.instruction_pointer >= self.script().len() {
			self.instruction_pointer = 0;
		}
	}

	pub fn clone(&self) -> Self {
		Self::clone_with_ip(self, self.instruction_pointer)
	}

	pub fn clone_with_ip(&self, ip: usize) -> Self {
		let shared_states = Rc::clone(&self.shared_states);

		Self {
			shared_states,
			instruction_pointer: ip,
			rv_count: 0,

			local_variables: self.local_variables.clone(),
			arguments: self.arguments.clone(),
			try_stack: self.try_stack.clone(),
		}
	}

	// Get the current instruction
	pub fn current_instruction(&self) -> &Instruction {
		self.script().get_instruction(self.instruction_pointer)?
	}

	pub fn next_instruction(&self) -> &Instruction {
		let next_ip = self.instruction_pointer + self.current_instruction().size();
		self.script().get_instruction(next_ip)?
	}
	
}
