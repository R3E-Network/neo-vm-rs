use crate::{
	exception_handling_context::ExceptionHandlingContext,
	exception_handling_state::ExceptionHandlingState, execution_engine::ExecutionEngine,
	instruction::Instruction, jump_table::JumpTable, types::stack_item::StackItem,
	vm_state::VMState,
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};
impl JumpTable {
	/// No operation. Does nothing.
	/// <see cref="OpCode::NOP"/>
	pub fn nop(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		// Do nothing
	}

	/// Unconditionally transfers control to a target instruction.
	/// <see cref="OpCode::JMP"/>
	pub fn jmp(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.execute_jump_offset(engine, instruction.token_i8() as i32);
	}

	/// Unconditionally transfers control to a target instruction (4-byte offset).
	/// <see cref="OpCode::JMP_L"/>
	pub fn jmp_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.execute_jump_offset(engine, instruction.token_i32());
	}

	/// Transfers control to a target instruction if the value is true, not null, or non-zero.
	/// <see cref="OpCode::JMPIF"/>
	pub fn jmp_if(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if engine.pop().unwrap().borrow().get_boolean() {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the value is true, not null, or non-zero (4-byte offset).
	/// <see cref="OpCode::JMPIF_L"/>
	pub fn jmp_if_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if engine.pop().unwrap().borrow().get_boolean() {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if the value is false, a null reference, or zero.
	/// <see cref="OpCode::JMPIFNOT"/>
	pub fn jmp_if_not(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if !engine.pop().unwrap().borrow().get_boolean() {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the value is false, a null reference, or zero (4-byte offset).
	/// <see cref="OpCode::JMPIFNOT_L"/>
	pub fn jmp_if_not_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if !engine.pop().unwrap().borrow().get_boolean() {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if two values are equal.
	/// <see cref="OpCode::JMPEQ"/>
	pub fn jmp_eq(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 == x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if two values are equal (4-byte offset).
	/// <see cref="OpCode::JMPEQ_L"/>
	pub fn jmp_eq_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 == x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction when two values are not equal.
	/// <see cref="OpCode::JMPNE"/>
	pub fn jmp_ne(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 != x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction when two values are not equal (4-byte offset).
	/// <see cref="OpCode::JMPNE_L"/>
	pub fn jmp_ne_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 != x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if the first value is greater than the second value.
	/// <see cref="OpCode::JMPGT"/>
	pub fn jmp_gt(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 > x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the first value is greater than the second value (4-byte offset).
	/// <see cref="OpCode::JMPGT_L"/>
	pub fn jmp_gt_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 > x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if the first value is greater than or equal to the second value.
	/// <see cref="OpCode::JMPGE"/>
	pub fn jmp_ge(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 >= x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the first value is greater than or equal to the second value (4-byte offset).
	/// <see cref="OpCode::JMPGE_L"/>
	pub fn jmp_ge_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 >= x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if the first value is less than the second value.
	/// <see cref="OpCode::JMPLT"/>
	pub fn jmp_lt(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 < x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the first value is less than the second value (4-byte offset).
	/// <see cref="OpCode::JMPLT_L"/>
	pub fn jmp_lt_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 < x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Transfers control to a target instruction if the first value is less than or equal to the second value.
	/// <see cref="OpCode::JMPLE"/>
	pub fn jmp_le(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 <= x2 {
			self.execute_jump_offset(engine, instruction.token_i8() as i32);
		}
	}

	/// Transfers control to a target instruction if the first value is less than or equal to the second value (4-byte offset).
	/// <see cref="OpCode::JMPLE_L"/>
	pub fn jmp_le_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x2 = engine.pop().unwrap().borrow().get_integer();
		let x1 = engine.pop().unwrap().borrow().get_integer();
		if x1 <= x2 {
			self.execute_jump_offset(engine, instruction.token_i32());
		}
	}

	/// Calls the function at the target address.
	/// <see cref="OpCode::CALL"/>
	pub fn call(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.execute_call(
			engine,
			match engine
				.current_context()
				.unwrap()
				.instruction_pointer
				.checked_add(instruction.token_i8() as i32)
			{
				Some(result) => result,
				None => {
					engine.state = VMState::Fault;
					return;
				},
			},
		);
	}

	/// Calls the function at the target address (4-byte offset).
	/// <see cref="OpCode::CALL_L"/>
	pub fn call_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.execute_call(
			engine,
			match engine
				.current_context()
				.unwrap()
				.instruction_pointer
				.checked_add(instruction.token_i32())
			{
				Some(result) => result,
				None => {
					engine.state = VMState::Fault;
					return;
				},
			},
		);
	}

	/// Pop the address of a function from the stack, and call the function.
	/// <see cref="OpCode::CALLA"/>
	pub fn call_a(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let x = match engine.pop() {
			Some(x) => x,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		if x.script != engine.current_context().unwrap().script {
			engine.state = VMState::Fault;
			return;
		}
		self.execute_call(engine, x.position);
	}

	/// Calls the function which is described by the token.
	/// <see cref="OpCode::CALLT"/>
	pub fn call_t(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_token(engine, instruction.token_u16());
	}

	/// It turns the vm state to FAULT immediately, and cannot be caught.
	/// <see cref="OpCode::ABORT"/>
	pub fn abort(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		engine.state = VMState::Fault;
	}

	/// Pop the top value of the stack. If it's false, exit vm execution and set vm state to FAULT.
	/// <see cref="OpCode::ASSERT"/>
	pub fn assert(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if !engine.pop().unwrap().borrow().get_boolean() {
			engine.state = VMState::Fault;
		}
	}

	/// Pop the top value of the stack, and throw it.
	/// <see cref="OpCode::THROW"/>
	pub fn throw(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.execute_throw(engine, engine.pop());
	}

	/// TRY CatchOffset(sbyte) FinallyOffset(sbyte)
	/// <see cref="OpCode::TRY"/>
	pub fn try_op(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let catch_offset = instruction.token_i8() as i32;
		let finally_offset = instruction.token_i8_1() as i32;
		self.execute_try(engine, catch_offset, finally_offset);
	}

	/// TRY_L CatchOffset(int) FinallyOffset(int)
	/// <see cref="OpCode::TRY_L"/>
	pub fn try_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let catch_offset = instruction.token_i32();
		let finally_offset = instruction.token_i32_1();
		self.execute_try(engine, catch_offset, finally_offset);
	}

	/// Ensures that the appropriate surrounding finally blocks are executed.
	/// <see cref="OpCode::ENDTRY"/>
	pub fn end_try(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let end_offset = instruction.token_i8() as i32;
		self.execute_end_try(engine, end_offset);
	}

	/// Ensures that the appropriate surrounding finally blocks are executed (4-byte offset).
	/// <see cref="OpCode::ENDTRY_L"/>
	pub fn end_try_l(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let end_offset = instruction.token_i32();
		self.execute_end_try(engine, end_offset);
	}

	/// End finally, If no exception happen or be catched, vm will jump to the target instruction of ENDTRY/ENDTRY_L.
	/// <see cref="OpCode::ENDFINALLY"/>
	pub fn end_finally(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if engine.current_context().unwrap().try_stack.is_none() {
			engine.state = VMState::Fault;
			return;
		}
		let current_try = match engine.current_context().unwrap().try_stack.as_mut().unwrap().pop()
		{
			Some(try_context) => try_context,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};

		if let ExceptionHandlingState::Finally = current_try.state() {
			engine.state = VMState::Fault;
			return;
		}

		if engine.uncaught_exception.is_none() {
			engine.current_context().unwrap().instruction_pointer = current_try.end_pointer;
		} else {
			self.execute_throw(engine, engine.uncaught_exception.take().unwrap());
		}

		engine.is_jumping = true;
	}

	/// Returns from the current method.
	/// <see cref="OpCode::RET"/>
	pub fn ret(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let mut context_pop = engine.invocation_stack.pop().unwrap();
		let stack_eval = match engine.invocation_stack.len() == 0 {
			true => engine.result_stack.clone(),
			false => engine.invocation_stack.last().unwrap().borrow().evaluation_stack().clone(),
		};
		if context_pop.borrow().evaluation_stack() != stack_eval {
			if context_pop.borrow().rv_count >= 0
				&& context_pop.borrow_mut().evaluation_stack().len()
					!= context_pop.borrow().rv_count as usize
			{
				return Err(VMState::Fault);
			}
			context_pop.borrow_mut().evaluation_stack().copy_to(stack_eval, None);
		}
		if engine.invocation_stack.len() == 0 {
			engine.state = VMState::HALT;
		}

		engine.unload_context(context_pop);
		engine.is_jumping = true;
	}

	/// Calls to an interop service.
	/// <see cref="OpCode::SYSCALL"/>
	pub fn syscall(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		// This is typically implemented by the specific VM implementation
		// as it depends on the available system calls
		unimplemented!("Syscall not implemented");
	}

	// Helper methods
	fn execute_call(&self, engine: &mut ExecutionEngine, position: i32) {
		engine.load_context(engine.current_context().unwrap().clone_at_offset(position));
	}

	fn execute_jump_offset(&self, engine: &mut ExecutionEngine, offset: i32) {
		self.execute_jump(
			engine,
			match engine.current_context().unwrap().instruction_pointer.checked_add(offset) {
				Some(result) => result,
				None => {
					engine.state = VMState::Fault;
					return;
				},
			},
		);
	}

	fn execute_jump(&self, engine: &mut ExecutionEngine, position: i32) {
		if position < 0 || position >= engine.current_context().unwrap().script.len() as i32 {
			return Err(VMState::Fault);
		}
		engine.current_context().unwrap().instruction_pointer = position as usize;
		engine.is_jumping = true;
	}

	fn execute_try(&self, engine: &mut ExecutionEngine, catch_offset: i32, finally_offset: i32) {
		if catch_offset == 0 && finally_offset == 0 {
			return Err(VMState::Fault);
		}
		if engine.current_context().unwrap().try_stack.is_none() {
			engine.current_context().unwrap().try_stack = Some(Vec::new());
		} else if engine.current_context().unwrap().try_stack.as_ref().unwrap().len()
			>= engine.limits.max_try_nesting_depth
		{
			return Err(VMState::Fault);
		}
		let catch_pointer = if catch_offset > 0 {
			Some(
				engine.current_context().unwrap().borrow_mut().instruction_pointer
					+ catch_offset as usize,
			)
		} else {
			None
		};
		let finally_pointer = if finally_offset > 0 {
			match engine
				.current_context()
				.unwrap()
				.instruction_pointer
				.checked_add(finally_offset as usize)
			{
				Some(result) => result,
				None => {
					engine.state = VMState::Fault;
					return;
				},
			}
		} else {
			None
		};
		engine
			.current_context()
			.unwrap()
			.try_stack
			.as_mut()
			.unwrap()
			.push(ExceptionHandlingContext::new(catch_pointer, finally_pointer));
	}

	fn execute_end_try(&self, engine: &mut ExecutionEngine, end_offset: i32) {
		if engine.current_context().unwrap().try_stack.is_none() {
			return Err(VMState::Fault);
		}
		let current_try =
			match engine.current_context().unwrap().try_stack.as_mut().unwrap().last_mut() {
				Some(try_context) => try_context,
				None => return Err(VMState::Fault),
			};
		if current_try.state() == ExceptionHandlingState::Finally {
			return Err(VMState::Fault);
		}
		let end_pointer = match engine
			.current_context()
			.unwrap()
			.instruction_pointer
			.checked_add(end_offset as usize)
		{
			Some(result) => result,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		if current_try.finally_pointer().is_some() {
			current_try.set_state(ExceptionHandlingState::Finally);
			current_try.set_end_pointer(end_pointer);
			engine.current_context().unwrap().instruction_pointer =
				current_try.finally_pointer().unwrap();
		} else {
			engine.current_context().unwrap().try_stack.as_mut().unwrap().pop();
			engine.current_context().unwrap().instruction_pointer = end_pointer;
		}
		engine.is_jumping = true;
	}

	fn execute_throw(&self, engine: &mut ExecutionEngine, exception: Rc<RefCell<StackItem>>) {
		engine.uncaught_exception = Some(exception);
		let mut pop = 0;
		for context in engine.invocation_stack.iter().rev() {
			if let Some(try_stack) = &mut context.borrow_mut().try_stack {
				while let Some(try_context) = try_stack.last_mut() {
					if try_context.state() == ExceptionHandlingState::Finally
						|| (try_context.state() == ExceptionHandlingState::Catch
							&& try_context.finally_pointer().is_none())
					{
						try_stack.pop();
						continue;
					}
					for _ in 0..pop {
						engine.unload_context(engine.invocation_stack.pop().unwrap());
					}
					if try_context.state() == ExceptionHandlingState::Try
						&& try_context.catch_pointer().is_some()
					{
						try_context.set_state(ExceptionHandlingState::Catch);
						engine.push(engine.uncaught_exception.take().unwrap());
						context.borrow_mut().instruction_pointer =
							try_context.catch_pointer().unwrap();
						engine.uncaught_exception = None;
					} else {
						try_context.set_state(ExceptionHandlingState::Finally);
						context.borrow_mut().instruction_pointer =
							try_context.finally_pointer().unwrap();
					}
					engine.is_jumping = true;
					return;
				}
			}
			pop += 1;
		}
		// If we get here, the exception was not caught
		engine.state = VMState::Fault;
	}

	fn load_token(&self, engine: &mut ExecutionEngine, token: u16) {
		// This is typically implemented by the specific VM implementation
		// as it depends on how tokens are handled
		unimplemented!("Load token not implemented");
	}
}
