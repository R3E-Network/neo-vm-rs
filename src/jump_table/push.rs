use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable,
	types::stack_item::StackItem, vm_state::VMState,
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};
impl JumpTable {
	/// Pushes a 1-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT8"/>
	pub fn push_int8(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from(instruction.token_i8());
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes a 2-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT16"/>
	pub fn push_int16(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from(instruction.token_i16());
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes a 4-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT32"/>
	pub fn push_int32(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from(instruction.token_i32());
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes an 8-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT64"/>
	pub fn push_int64(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from(instruction.token_i64());
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes a 16-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT128"/>
	pub fn push_int128(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from_signed_bytes_le(&instruction.operand);
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes a 32-byte signed integer onto the stack.
	/// <see cref="OpCode::PUSHINT256"/>
	pub fn push_int256(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let value = BigInt::from_signed_bytes_le(&instruction.operand);
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value))));
	}

	/// Pushes the boolean value true onto the stack.
	/// <see cref="OpCode::PUSHTRUE"/>
	pub fn push_true(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(true))));
	}

	/// Pushes the boolean value false onto the stack.
	/// <see cref="OpCode::PUSHFALSE"/>
	pub fn push_false(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(false))));
	}

	/// Converts the 4-byte offset to a pointer, and pushes it onto the stack.
	/// <see cref="OpCode::PUSHA"/>
	pub fn push_a(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let offset = instruction.token_i32();
		let position = engine.current_context().unwrap().borrow().instruction_pointer as i32 + offset;
		if position < 0 || position as usize > engine.current_context().unwrap().borrow().script().len() {
			engine.state = VMState::Fault;
			return;
		}
		let pointer = Rc::new(RefCell::new(StackItem::Pointer(
			engine.current_context().unwrap().borrow().script().clone(),
			position as usize,
		)));
		engine.push(pointer);
	}

	/// Pushes null onto the stack.
	/// <see cref="OpCode::PUSHNULL"/>
	pub fn push_null(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Null)));
	}

	/// The next byte contains the number of bytes to be pushed onto the stack.
	/// <see cref="OpCode::PUSHDATA1"/>
	pub fn push_data1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.push_data(engine, &instruction.operand)
	}

	/// The next two bytes contain the number of bytes to be pushed onto the stack.
	/// <see cref="OpCode::PUSHDATA2"/>
	pub fn push_data2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.push_data(engine, &instruction.operand)
	}

	/// The next four bytes contain the number of bytes to be pushed onto the stack.
	/// <see cref="OpCode::PUSHDATA4"/>
	pub fn push_data4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.push_data(engine, &instruction.operand)
	}

	/// Pushes the number -1 onto the stack.
	/// <see cref="OpCode::PUSHM1"/>
	pub fn push_m1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(-1)))));
	}

	/// Pushes the number 0 onto the stack.
	/// <see cref="OpCode::PUSH0"/>
	pub fn push0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(0)))));
	}

	/// Pushes the number 1 onto the stack.
	/// <see cref="OpCode::PUSH1"/>
	pub fn push1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(1)))));
	}

	/// Pushes the number 2 onto the stack.
	/// <see cref="OpCode::PUSH2"/>
	pub fn push2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(2)))));
	}

	/// Pushes the number 3 onto the stack.
	/// <see cref="OpCode::PUSH3"/>
	pub fn push3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(3)))));
	}

	/// Pushes the number 4 onto the stack.
	/// <see cref="OpCode::PUSH4"/>
	pub fn push4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(4)))));
	}

	/// Pushes the number 5 onto the stack.
	/// <see cref="OpCode::PUSH5"/>
	pub fn push5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(5)))));
	}

	/// Pushes the number 6 onto the stack.
	/// <see cref="OpCode::PUSH6"/>
	pub fn push6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(6)))));
	}

	/// Pushes the number 7 onto the stack.
	/// <see cref="OpCode::PUSH7"/>
	pub fn push7(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(7)))));
	}

	/// Pushes the number 8 onto the stack.
	/// <see cref="OpCode::PUSH8"/>
	pub fn push8(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(8)))));
	}

	/// Pushes the number 9 onto the stack.
	/// <see cref="OpCode::PUSH9"/>
	pub fn push9(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(9)))));
	}

	/// Pushes the number 10 onto the stack.
	/// <see cref="OpCode::PUSH10"/>
	pub fn push10(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(10)))));
	}

	/// Pushes the number 11 onto the stack.
	/// <see cref="OpCode::PUSH11"/>
	pub fn push11(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(11)))));
	}

	/// Pushes the number 12 onto the stack.
	/// <see cref="OpCode::PUSH12"/>
	pub fn push12(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(12)))));
	}

	/// Pushes the number 13 onto the stack.
	/// <see cref="OpCode::PUSH13"/>
	pub fn push13(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(13)))));
	}

	/// Pushes the number 14 onto the stack.
	/// <see cref="OpCode::PUSH14"/>
	pub fn push14(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(14)))));
	}

	/// Pushes the number 15 onto the stack.
	/// <see cref="OpCode::PUSH15"/>
	pub fn push15(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(15)))));
	}

	/// Pushes the number 16 onto the stack.
	/// <see cref="OpCode::PUSH16"/>
	pub fn push16(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(16)))));
	}

	fn push_data(&self, engine: &mut ExecutionEngine, data: &[u8]) {
		if let Err(_) = engine.limits.assert_max_item_size(data.len()) {
			engine.state = VMState::Fault;
			return;
		}
		let item = if data.len() <= 1024 {
			StackItem::ByteString(data.to_vec())
		} else {
			StackItem::Buffer(data.to_vec())
		};
		engine.push(Rc::new(RefCell::new(item)));
	}
}
