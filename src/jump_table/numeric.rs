use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable, types::stack_item::StackItem, vm_state::VMState
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};
impl JumpTable {
	/// Computes the sign of the specified integer.
	/// If the value is negative, puts -1; if positive, puts 1; if zero, puts 0.
	/// <see cref="OpCode::SIGN"/>
	pub fn sign(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(x.signum())));
	}

	/// Computes the absolute value of the specified integer.
	/// <see cref="OpCode::ABS"/>
	pub fn abs(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x.abs()))));
	}

	/// Computes the negation of the specified integer.
	/// <see cref="OpCode::NEGATE"/>
	pub fn negate(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(-x))));
	}

	/// Increments the specified integer by one.
	/// <see cref="OpCode::INC"/>
	pub fn inc(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x + 1))));
	}

	/// Decrements the specified integer by one.
	/// <see cref="OpCode::DEC"/>
	pub fn dec(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x - 1))));
	}

	/// Computes the sum of two integers.
	/// <see cref="OpCode::ADD"/>
	pub fn add(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1 + x2))));
	}

	/// Computes the difference between two integers.
	/// <see cref="OpCode::SUB"/>
	pub fn sub(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
				engine.push(Rc::new(RefCell::new(StackItem::Integer(x1 - x2))));
	}

	/// Computes the product of two integers.
	/// <see cref="OpCode::MUL"/>
	pub fn mul(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1 * x2))));
	}

	/// Computes the quotient of two integers.
	/// <see cref="OpCode::DIV"/>
	pub fn div(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		if x2 == BigInt::from(0) {
			engine.state = VMState::Fault;
			return;
		}
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1 / x2))));
	}

	/// Computes the remainder after dividing a by b.
	/// <see cref="OpCode::MOD"/>
	pub fn mod_op(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		if x2 == BigInt::from(0) {
			engine.state = VMState::Fault;
			return;
		}
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1 % x2)	)));
	}

	/// Computes the result of raising a number to the specified power.
	/// <see cref="OpCode::POW"/>
	pub fn pow(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let exponent = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.limits.assert_shift(exponent.to_i32().unwrap());
		let value = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(value.pow(exponent as u32)	))));
	}

	/// Returns the square root of a specified number.
	/// <see cref="OpCode::SQRT"/>
	pub fn sqrt(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x.sqrt()))));
	}

	/// Computes the modular multiplication of two integers.
	/// <see cref="OpCode::MODMUL"/>
	pub fn mod_mul(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let modulus = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer((x1 * x2) % modulus))));
	}

	/// Computes the modular exponentiation of an integer.
	/// <see cref="OpCode::MODPOW"/>
	pub fn mod_pow(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let modulus = engine.pop().unwrap().borrow().get_integer().unwrap();
		let exponent = engine.pop().unwrap().borrow().get_integer().unwrap();
		let value = engine.pop().unwrap().borrow().get_integer().unwrap();
		let result = if exponent == BigInt::from(-1) {
			value.mod_inverse(&modulus).unwrap_or_else(|| BigInt::from(0))
		} else {
			value.modpow(&exponent, &modulus)
		};
		engine.push(Rc::new(RefCell::new(StackItem::Integer(result))));
	}

	/// Computes the left shift of an integer.
	/// <see cref="OpCode::SHL"/>
	pub fn shl(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let shift = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.limits.assert_shift(shift);
		if shift == 0 {
			return;
		}
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x << shift))));
	}

	/// Computes the right shift of an integer.
	/// <see cref="OpCode::SHR"/>
	pub fn shr(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let shift = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.limits.assert_shift(shift);
		if shift == 0 {
			return;
		}
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x >> shift))));
	}

	/// If the input is 0 or 1, it is flipped. Otherwise the output will be 0.
	/// <see cref="OpCode::NOT"/>
	pub fn not(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(!x))));
	}

	/// Computes the logical AND of the top two stack items and pushes the result onto the stack.
	/// <see cref="OpCode::BOOLAND"/>
	pub fn bool_and(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1 && x2))));
	}

	/// Computes the logical OR of the top two stack items and pushes the result onto the stack.
	/// <see cref="OpCode::BOOLOR"/>
	pub fn bool_or(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1 || x2))));
	}

	/// Determines whether the top stack item is not zero and pushes the result onto the stack.
	/// <see cref="OpCode::NZ"/>
	pub fn nz(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(!x.is_zero()))));
	}

	/// Determines whether the top two stack items are equal and pushes the result onto the stack.
	/// <see cref="OpCode::NUMEQUAL"/>
	pub fn num_equal(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().get_integer();
		let x1 = engine.pop().get_integer();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1 == x2))));
	}

	/// Determines whether the top two stack items are not equal and pushes the result onto the stack.
	/// <see cref="OpCode::NUMNOTEQUAL"/>
	pub fn num_not_equal(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();

		engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1 != x2))));
	}

	/// Determines whether the first value is less than the second value.
	/// <see cref="OpCode::LT"/>
	pub fn lt(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow();
		let x1 = engine.pop().unwrap().borrow();
		if x1.is_null() || x2.is_null() {
			engine.push(Rc::new(RefCell::new(false)));
		} else {
			engine.push(Rc::new(RefCell::new(x1.get_integer() < x2.get_integer())));
		}
	}

	/// Determines whether the first value is less than or equal to the second value.
	/// <see cref="OpCode::LE"/>
	pub fn le(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow();
		let x1 = engine.pop().unwrap().borrow();
		if x1.is_null() || x2.is_null() {
			engine.push(Rc::new(RefCell::new(false)));
		} else {
			engine.push(Rc::new(RefCell::new(x1.get_integer() <= x2.get_integer())));
		}
	}

	/// Determines whether the first value is greater than the second value.
	/// <see cref="OpCode::GT"/>
	pub fn gt(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow();
		let x1 = engine.pop().unwrap().borrow();
		if x1.is_null() || x2.is_null() {
			engine.push(Rc::new(RefCell::new(StackItem::Boolean(false))));
		} else {
			engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1.get_integer() > x2.get_integer()))));
		}
	}

	/// Determines whether the first value is greater than or equal to the second value.
	/// <see cref="OpCode::GE"/>
	pub fn ge(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow();
		let x1 = engine.pop().unwrap().borrow();
		if x1.is_null() || x2.is_null() {
			engine.push(Rc::new(RefCell::new(StackItem::Boolean(false))));
		} else {
			engine.push(Rc::new(RefCell::new(StackItem::Boolean(x1.get_integer() >= x2.get_integer()))));
		}
	}

	/// Computes the minimum of the top two stack items and pushes the result onto the stack.
	/// <see cref="OpCode::MIN"/>
	pub fn min(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1.min(x2)))));
	}

	/// Computes the maximum of the top two stack items and pushes the result onto the stack.
	/// <see cref="OpCode::MAX"/>
	pub fn max(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x1 = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Integer(x1.max(x2)))));
	}

	/// Determines whether the top stack item is within the range specified by the next two top stack items
	/// and pushes the result onto the stack.
	/// <see cref="OpCode::WITHIN"/>
	pub fn within(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let b = engine.pop().unwrap().borrow().get_integer().unwrap();		
		let a = engine.pop().unwrap().borrow().get_integer().unwrap();
		let x = engine.pop().unwrap().borrow().get_integer().unwrap();
		engine.push(Rc::new(RefCell::new(StackItem::Boolean(a <= x && x < b))));
	}
}
