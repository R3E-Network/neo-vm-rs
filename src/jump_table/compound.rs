use crate::{
	execution_engine::ExecutionEngine,
	instruction::Instruction,
	jump_table::JumpTable,
	types::{stack_item::StackItem, stack_item_type::StackItemType},
	vm_state::VMState,
};
use num_bigint::BigInt;
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

impl JumpTable {
	/// Packs a map from the evaluation stack.
	/// <see cref="OpCode::PACKMAP"/>
	pub fn pack_map(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let size = match engine.pop().get_integer().and_then(|i| i.to_usize()) {
			Some(s) => s,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if size * 2 > engine.current_context().unwrap().evaluation_stack.len() {
			engine.state = VMState::Fault;
			return;
		}
		let map = Rc::new(RefCell::new(HashMap::new()));
		for _ in 0..size {
			let key = engine.pop();
			let value = engine.pop();
			map.borrow_mut().insert(key, value);
		}
		engine.push(map);
	}

	/// Packs a struct from the evaluation stack.
	/// <see cref="OpCode::PACKSTRUCT"/>
	pub fn pack_struct(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let size = match engine.pop().get_integer().and_then(|i| i.to_usize()) {
			Some(s) => s,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if size > engine.current_context().unwrap().evaluation_stack.len() {
			engine.state = VMState::Fault;
			return;
		}
		let struct_ = Rc::new(RefCell::new(Vec::new()));
		for _ in 0..size {
			let item = engine.pop();
			struct_.borrow_mut().push(item);
		}
		engine.push(struct_);
	}

	/// Packs an array from the evaluation stack.
	/// <see cref="OpCode::PACK"/>
	pub fn pack(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let size = match engine.pop().get_integer().and_then(|i| i.to_usize()) {
			Some(s) => s,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if size > engine.current_context().unwrap().evaluation_stack.len() {
			engine.state = VMState::Fault;
			return;
		}
		let array = Rc::new(RefCell::new(StackItem::Array(Vec::new())));
		for _ in 0..size {
			let item = engine.pop();
			if let StackItem::Array(ref mut vec) = *array.borrow_mut() {
				vec.push(item);
			}
		}
		engine.push(array);
	}

	/// Unpacks a compound type from the evaluation stack.
	/// <see cref="OpCode::UNPACK"/>
	pub fn unpack(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let compound = match engine.pop() {
			Some(c) => c,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		match &*compound.borrow_mut() {
			StackItem::Map(map) => {
				for (key, value) in map.iter() {
					engine.push(Rc::clone(value));
					engine.push(Rc::new(RefCell::new(key.clone())));
				}
			},
			StackItem::Array(array) | StackItem::Struct(array) => {
				for item in array.iter() {
					engine.push(Rc::clone(item));
				}
			},
			_ => {
				engine.state = VMState::Fault;
				return;
			}
		}
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(compound.borrow().len())))));
	}

	/// Creates a new empty array with zero elements on the evaluation stack.
	/// <see cref="OpCode::NEWARRAY0"/>
	pub fn new_array0(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		engine.push(Rc::new(RefCell::new(StackItem::Array(Vec::new()))));
	}

	/// Creates a new array with a specified number of elements on the evaluation stack.
	/// <see cref="OpCode::NEWARRAY"/>
	pub fn new_array(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let n = match engine.pop().and_then(|item| item.borrow_mut().get_integer()) {
			Some(n) => n,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if n > engine.limits.max_stack_size {
			engine.state = VMState::Fault;
			return;
		}
		let array = Rc::new(RefCell::new(StackItem::Array(vec![
			Rc::new(RefCell::new(StackItem::Null));
			n.to_usize().unwrap_or(0)
		])));
		engine.push(array);
	}

	/// Creates a new array with a specified number of elements and a specified type on the evaluation stack.
	/// <see cref="OpCode::NEWARRAY_T"/>
	pub fn new_array_t(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let n = match engine.pop().get_integer().and_then(|i| i.to_usize()) {
			Some(n) => n,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if n > engine.limits.max_stack_size {
			engine.state = VMState::Fault;
			return;
		}
		let type_ = StackItemType::from(instruction.token_u8());
		if !StackItemType::is_valid(&type_) {
			engine.state = VMState::Fault;
			return;
		}
		let item = match type_ {
			StackItemType::Boolean => Rc::new(RefCell::new(StackItem::Boolean(false))),
			StackItemType::Integer => Rc::new(RefCell::new(StackItem::Integer(BigInt::from(0)))),
			StackItemType::ByteString => Rc::new(RefCell::new(StackItem::ByteString(Vec::new()))),
			_ => Rc::new(RefCell::new(StackItem::Null)),
		};
		let array = Rc::new(RefCell::new(StackItem::Array(vec![Rc::clone(&item); n])));
		engine.push(array);
	}

	/// Creates a new empty struct with zero elements on the evaluation stack.
	/// <see cref="OpCode::NEWSTRUCT0"/>
	pub fn new_struct0(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		engine.push(Rc::new(RefCell::new(StackItem::Struct(Vec::new()))));
	}

	/// Creates a new struct with a specified number of elements on the evaluation stack.
	/// <see cref="OpCode::NEWSTRUCT"/>
	pub fn new_struct(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let n = match engine.pop().get_integer().and_then(|i| i.to_usize()) {
			Some(n) => n,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		if n > engine.limits.max_stack_size {
			engine.state = VMState::Fault;
			return;
		}
		let struct_ = Rc::new(RefCell::new(StackItem::Struct(vec![
			Rc::new(RefCell::new(StackItem::Null));
			n
		])));
		engine.push(struct_);
	}

	/// Creates a new empty map on the evaluation stack.
	/// <see cref="OpCode::NEWMAP"/>
	pub fn new_map(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		engine.push(Rc::new(RefCell::new(StackItem::Map(std::collections::HashMap::new()))));
	}

	/// Gets the size of the top item on the evaluation stack and pushes it onto the stack.
	/// <see cref="OpCode::SIZE"/>
	pub fn size(
		&self,
		engine: &mut ExecutionEngine,
		instruction: &Instruction,
	) {
		let x = match engine.pop() {
			Some(item) => item,
			None => {
				engine.state = VMState::Fault;
				return;
			}
		};
		let size = match &*x.borrow() {
			StackItem::Array(array) => array.len(),
			StackItem::Map(map) => map.len(),
			StackItem::Struct(struct_) => struct_.len(),
			StackItem::ByteString(bytes) => bytes.len(),
			StackItem::Buffer(buffer) => buffer.len(),
			StackItem::Integer(integer) => integer.to_bytes_le().1.len(),
			_ => {
				engine.state = VMState::Fault;
				return;
			}
		};
		engine.push(Rc::new(RefCell::new(StackItem::Integer(BigInt::from(size)))));
	}

	// ... (continued in the next message due to length constraints)
}
