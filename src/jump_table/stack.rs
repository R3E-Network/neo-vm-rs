use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable,
	types::stack_item::StackItem, vm_state::VMState,
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};

impl JumpTable {
	// Stack operations

	/// Puts the number of stack items onto the stack.
	/// <see cref="OpCode::DEPTH"/>
	pub fn depth(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let depth = engine
			.current_context()
			.as_ref()
			.unwrap()
			.borrow()
			.evaluation_stack()
			.borrow()
			.count();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(depth.into()))));
	}

	/// Removes the top stack item.
	/// <see cref="OpCode::DROP"/>
	pub fn drop(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.pop();
	}

	/// Removes the second-to-top stack item.
	/// <see cref="OpCode::NIP"/>
	pub fn nip(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let top = engine.pop().unwrap();
		engine.pop();
		engine.push(top);
	}

	/// The item n back in the main stack is removed.
	/// <see cref="OpCode::XDROP"/>
	pub fn xdrop(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let n = engine.pop().unwrap().borrow().get_integer().unwrap();
		if n >= engine.current_context().unwrap().borrow().evaluation_stack().borrow().count() {
			return Err(VMState::Fault);
		}
		engine.current_context().unwrap().borrow().evaluation_stack().borrow().remove(n);
	}

	/// Clear the stack
	/// <see cref="OpCode::CLEAR"/>
	pub fn clear(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.current_context().unwrap().evaluation_stack.clear();
	}

	/// Duplicates the top stack item.
	/// <see cref="OpCode::DUP"/>
	pub fn dup(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let item = engine.peek(0);
		engine.push(item);
	}

	/// Copies the second-to-top stack item to the top.
	/// <see cref="OpCode::OVER"/>
	pub fn over(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let item = engine.peek(1);
		engine.push(item);
	}

	/// The item n back in the stack is copied to the top.
	/// <see cref="OpCode::PICK"/>
	pub fn pick(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let n = engine.pop().get_integer().to_usize().unwrap();
		let item = engine.peek(n);
		engine.push(item);
	}

	/// The item at the top of the stack is copied and inserted before the second-to-top item.
	/// <see cref="OpCode::TUCK"/>
	pub fn tuck(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let top = engine.pop();
		let second = engine.pop();
		engine.push(top.clone());
		engine.push(second);
		engine.push(top);
	}

	/// The top two items on the stack are swapped.
	/// <see cref="OpCode::SWAP"/>
	pub fn swap(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let top = engine.pop();
		let second = engine.pop();
		engine.push(top);
		engine.push(second);
	}

	/// The top three items on the stack are rotated to the left.
	/// <see cref="OpCode::ROT"/>
	pub fn rot(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let top = engine.pop();
		let second = engine.pop();
		let third = engine.pop();
		engine.push(second);
		engine.push(top);
		engine.push(third);
	}

	/// The item n back in the stack is moved to the top.
	/// <see cref="OpCode::ROLL"/>
	pub fn roll(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let n = engine.pop().get_integer().to_usize().unwrap();
		if n == 0 {
			return;
		}
		let item = engine.current_context().unwrap().evaluation_stack.remove(n);
		engine.push(item);
	}

	/// Reverse the order of the top 3 items on the stack.
	/// <see cref="OpCode::REVERSE3"/>
	pub fn reverse3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.current_context().unwrap().evaluation_stack.reverse(3);
	}

	/// Reverse the order of the top 4 items on the stack.
	/// <see cref="OpCode::REVERSE4"/>
	pub fn reverse4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.current_context().unwrap().evaluation_stack.reverse(4);
	}

	/// Pop the number N on the stack, and reverse the order of the top N items on the stack.
	/// <see cref="OpCode::REVERSEN"/>
	pub fn reverse_n(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let n = engine.pop().get_integer().to_usize().unwrap();
		engine.current_context().unwrap().evaluation_stack.reverse(n);
	}
}
