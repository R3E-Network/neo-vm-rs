use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable,
	vm_state::VMState,
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};

impl JumpTable {
	/// Flips all of the bits of an integer.
	/// <see cref="OpCode::INVERT"/>
	pub fn invert(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().get_integer();
		engine.push(Rc::new(RefCell::new(!x)));
	}

	/// Computes the bitwise AND of two integers.
	/// <see cref="OpCode::AND"/>
	pub fn and(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().get_integer();
		let x1 = engine.pop().get_integer();
		engine.push(Rc::new(RefCell::new(x1 & x2)));
	}

	/// Computes the bitwise OR of two integers.
	/// <see cref="OpCode::OR"/>
	pub fn or(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().get_integer();
		let x1 = engine.pop().get_integer();
		engine.push(Rc::new(RefCell::new(x1 | x2)));
	}

	/// Computes the bitwise XOR (exclusive OR) of two integers.
	/// <see cref="OpCode::XOR"/>
	pub fn xor(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().get_integer();
		let x1 = engine.pop().get_integer();
		engine.push(Rc::new(RefCell::new(x1 ^ x2)));
	}

	/// Determines whether two objects are equal according to the execution engine's comparison rules.
	/// <see cref="OpCode::EQUAL"/>
	pub fn equal(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop();
		let x1 = engine.pop();
		engine.push(Rc::new(RefCell::new(x1.equals(&*x2, &engine.limits))));
	}

	/// Determines whether two objects are not equal according to the execution engine's comparison rules.
	/// <see cref="OpCode::NOTEQUAL"/>
	pub fn not_equal(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop();
		let x1 = engine.pop();
		engine.push(Rc::new(RefCell::new(!x1.equals(&*x2, &engine.limits))));
	}
}
