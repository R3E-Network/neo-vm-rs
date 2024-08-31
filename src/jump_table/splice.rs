use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable,
	types::stack_item::StackItem, vm_state::VMState,
};
use num_bigint::BigInt;
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

impl JumpTable {
	// Splice operations

	/// Creates a new Buffer with the size (number of bytes) specified by the value on the top of the stack.
	/// <see cref="OpCode::NEWBUFFER"/>
	pub fn new_buffer(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let size = match engine.pop().unwrap().borrow().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		engine.limits.assert_max_item_size(size);
		engine.push(Rc::new(RefCell::new(StackItem::Buffer(Vec::with_capacity(size)))));
	}

	/// Copies a range of bytes from one Buffer to another.
	/// <see cref="OpCode::MEMCPY"/>
	pub fn memcpy(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let count = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let src_index = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let src = match engine.pop().unwrap().borrow_mut().get_buffer() {
			Ok(b) => b,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let dst_index = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let mut dst = match engine.pop().unwrap().borrow_mut().get_buffer() {
			Ok(b) => b,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};

		if src_index.checked_add(count).unwrap() > src.len()
			|| dst_index.checked_add(count).unwrap() > dst.len()
		{
			engine.state = VMState::Fault;
			return;
		}

		dst[dst_index..dst_index + count].copy_from_slice(&src[src_index..src_index + count]);
	}

	/// Concatenates two strings.
	/// <see cref="OpCode::CAT"/>
	pub fn cat(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let b = match engine.pop().unwrap().borrow_mut().get_buffer_or_byte_string() {
			Ok(b) => b,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let a = match engine.pop().unwrap().borrow_mut().get_buffer_or_byte_string() {
			Ok(a) => a,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let result = [a, b].concat();
		engine.limits.assert_max_item_size(result.len());
		engine.push(Rc::new(RefCell::new(StackItem::ByteString(result.into()))));
	}

	/// Returns a section of a string.
	/// <see cref="OpCode::SUBSTR"/>
	pub fn substr(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let count = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let index = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let x = match engine.pop().unwrap().borrow_mut().get_buffer_or_byte_string() {
			Ok(x) => x,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};

		if index.checked_add(count).unwrap() > x.len() {
			engine.state = VMState::Fault;
			return;
		}

		engine.push(Rc::new(RefCell::new(StackItem::ByteString(x[index..index + count].to_vec()))));
	}

	/// Keeps only the first n bytes of a string.
	/// <see cref="OpCode::LEFT"/>
	pub fn left(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let count = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let x = match engine.pop().unwrap().borrow_mut().get_buffer_or_byte_string() {
			Ok(x) => x,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};

		if count > x.len() {
			engine.state = VMState::Fault;
			return;
		}

		engine.push(Rc::new(RefCell::new(StackItem::ByteString(x[..count].to_vec()))));
	}

	/// Keeps only the last n bytes of a string.
	/// <see cref="OpCode::RIGHT"/>
	pub fn right(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let count = match engine.pop().unwrap().borrow_mut().get_integer() {
			Ok(i) => i.to_usize().unwrap(),
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let x = match engine.pop().unwrap().borrow_mut().get_buffer_or_byte_string() {
			Ok(x) => x,
			Err(_) => {
				engine.state = VMState::Fault;
				return;
			}
		};

		if count > x.len() {
			engine.state = VMState::Fault;
			return;
		}

		engine.push(Rc::new(RefCell::new(StackItem::ByteString(x[x.len() - count..].to_vec()))));
	}
}
