use crate::{
	execution_engine::ExecutionEngine,
	instruction::Instruction,
	jump_table::JumpTable,
	types::{stack_item::StackItem, stack_item_type::StackItemType},
	vm_state::VMState,
};
use std::{cell::RefCell, rc::Rc};

impl JumpTable {
	/// Returns true if the input is null. Returns false otherwise.
	/// <see cref="OpCode::ISNULL"/>
	pub fn is_null(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap();
		let is_null = match &*x.borrow() {
			StackItem::Null => true,
			_ => false,
		};
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(is_null))));
	}

	/// Returns true if the top item is of the specified type.
	/// <see cref="OpCode::ISTYPE"/>
	pub fn is_type(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop();
		let type_ = StackItemType::from(instruction.token_u8());
		if type_ == StackItemType::Any || !StackItemType::is_valid_type(type_) {
			engine.state = VMState::Fault;
			return;
		}
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(x.get_type() == type_))));
	}

	/// Converts the top item to the specified type.
	/// <see cref="OpCode::CONVERT"/>
	pub fn convert(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop();
		let type_ = StackItemType::from(instruction.token_u8());
		engine.push(x.convert_to(type_)?);
	}
}
