use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, jump_table::JumpTable, slot::Slot,
	vm_state::VMState,
};
use num_bigint::BigInt;
use std::{cell::RefCell, rc::Rc};
impl JumpTable {
	/// Initialize the static field list for the current execution context.
	/// <see cref="OpCode::INITSSLOT"/>
	pub fn init_static_slot(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let count = instruction.token_u8() as usize;
		if count == 0 {
			engine.state = VMState::Fault;
			return;
		}
		if engine.current_context().unwrap().static_fields.is_some() {
			engine.state = VMState::Fault;
			return;
		}
		engine.current_context().unwrap().borrow().static_fields =
			Some(Slot::new(count, Some(Rc::clone(&engine.reference_counter))));
	}

	/// Initialize the argument slot and the local variable list for the current execution context.
	/// <see cref="OpCode::INITSLOT"/>
	pub fn init_slot(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		let local_count = instruction.token_u8() as usize;
		let arg_count = instruction.token_u8_1() as usize;
		if local_count + arg_count > 0 {
			if engine.current_context().unwrap().local_variables.is_some()
				|| engine.current_context().unwrap().arguments.is_some()
			{
				engine.state = VMState::Fault;
				return;
			}
			if local_count > 0 {
				engine.current_context().unwrap().local_variables =
					Some(Slot::new(local_count, Some(Rc::clone(&engine.reference_counter))));
			}
			if arg_count > 0 {
				let mut args = Vec::with_capacity(arg_count);
				for _ in 0..arg_count {
					args.push(engine.pop());
				}
				args.reverse();
				engine.current_context().unwrap().arguments =
					Some(Slot::new_with_items(args, Some(Rc::clone(&engine.reference_counter))));
			}
		}
	}

	/// Loads the static field at index 0 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD0"/>
	pub fn load_static_field_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 0);
	}

	/// Loads the static field at index 1 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD1"/>
	pub fn load_static_field_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 1);
	}

	/// Loads the static field at index 2 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD2"/>
	pub fn load_static_field_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 2);
	}

	/// Loads the static field at index 3 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD3"/>
	pub fn load_static_field_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 3);
	}

	/// Loads the static field at index 4 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD4"/>
	pub fn load_static_field_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 4);
	}

	/// Loads the static field at index 5 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD5"/>
	pub fn load_static_field_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 5);
	}

	/// Loads the static field at index 6 onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD6"/>
	pub fn load_static_field_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_static_field(engine, 6);
	}

	/// Loads the static field at a specified index onto the evaluation stack.
	/// <see cref="OpCode::LDSFLD"/>
	pub fn load_static_field(&self, engine: &mut ExecutionEngine, index: usize) {
		let static_fields = match engine.current_context().unwrap().static_fields.as_ref() {
			Some(fields) => fields,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		let item = match static_fields.get(index) {
			Some(item) => item,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		engine.push(item);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 0.
	/// <see cref="OpCode::STSFLD0"/>
	pub fn store_static_field_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 0);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 1.
	/// <see cref="OpCode::STSFLD1"/>
	pub fn store_static_field_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 1);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 2.
	/// <see cref="OpCode::STSFLD2"/>
	pub fn store_static_field_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 2);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 3.
	/// <see cref="OpCode::STSFLD3"/>
	pub fn store_static_field_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 3);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 4.
	/// <see cref="OpCode::STSFLD4"/>
	pub fn store_static_field_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 4);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 5.
	/// <see cref="OpCode::STSFLD5"/>
	pub fn store_static_field_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 5);
	}

	/// Stores the value on top of the evaluation stack in the static field list at index 6.
	/// <see cref="OpCode::STSFLD6"/>
	pub fn store_static_field_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_static_field(engine, 6);
	}

	/// Stores the value on top of the evaluation stack in the static field list at a specified index.
	/// <see cref="OpCode::STSFLD"/>
	pub fn store_static_field(&self, engine: &mut ExecutionEngine, index: usize) {
		let item = engine.pop();
		let static_fields = match engine.current_context().unwrap().static_fields.as_mut() {
			Some(fields) => fields,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		if static_fields.set(index, item).is_err() {
			engine.state = VMState::Fault;
		}
	}

	/// Loads the local variable at index 0 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC0"/>
	pub fn load_local_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 0);
	}

	/// Loads the local variable at index 1 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC1"/>
	pub fn load_local_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 1);
	}

	/// Loads the local variable at index 2 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC2"/>
	pub fn load_local_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 2);
	}

	/// Loads the local variable at index 3 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC3"/>
	pub fn load_local_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 3);
	}

	/// Loads the local variable at index 4 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC4"/>
	pub fn load_local_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 4);
	}

	/// Loads the local variable at index 5 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC5"/>
	pub fn load_local_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 5);
	}

	/// Loads the local variable at index 6 onto the evaluation stack.
	/// <see cref="OpCode::LDLOC6"/>
	pub fn load_local_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_local(engine, 6);
	}

	/// Loads the local variable at a specified index onto the evaluation stack.
	/// <see cref="OpCode::LDLOC"/>
	pub fn load_local(&self, engine: &mut ExecutionEngine, index: usize) {
		let local_variables = match engine.current_context().unwrap().local_variables.as_ref() {
			Some(variables) => variables,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		let item = match local_variables.get(index) {
			Some(item) => item,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		engine.push(item);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 0.
	/// <see cref="OpCode::STLOC0"/>
	pub fn store_local_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 0);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 1.
	/// <see cref="OpCode::STLOC1"/>
	pub fn store_local_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 1);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 2.
	/// <see cref="OpCode::STLOC2"/>
	pub fn store_local_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 2);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 3.
	/// <see cref="OpCode::STLOC3"/>
	pub fn store_local_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 3);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 4.
	/// <see cref="OpCode::STLOC4"/>
	pub fn store_local_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 4);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 5.
	/// <see cref="OpCode::STLOC5"/>
	pub fn store_local_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 5);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at index 6.
	/// <see cref="OpCode::STLOC6"/>
	pub fn store_local_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_local(engine, 6);
	}

	/// Stores the value on top of the evaluation stack in the local variable list at a specified index.
	/// <see cref="OpCode::STLOC"/>
	pub fn store_local(&self, engine: &mut ExecutionEngine, index: usize) {
		let item = engine.pop();
		let local_variables = match engine.current_context().unwrap().local_variables.as_mut() {
			Some(variables) => variables,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		if local_variables.set(index, item).is_err() {
			engine.state = VMState::Fault;
		}
	}

	/// Loads the argument at index 0 onto the evaluation stack.
	/// <see cref="OpCode::LDARG0"/>
	pub fn load_arg_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 0);
	}

	/// Loads the argument at index 1 onto the evaluation stack.
	/// <see cref="OpCode::LDARG1"/>
	pub fn load_arg_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 1);
	}

	/// Loads the argument at index 2 onto the evaluation stack.
	/// <see cref="OpCode::LDARG2"/>
	pub fn load_arg_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 2);
	}

	/// Loads the argument at index 3 onto the evaluation stack.
	/// <see cref="OpCode::LDARG3"/>
	pub fn load_arg_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 3);
	}

	/// Loads the argument at index 4 onto the evaluation stack.
	/// <see cref="OpCode::LDARG4"/>
	pub fn load_arg_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 4);
	}
	/// Loads the argument at index 5 onto the evaluation stack.
	/// <see cref="OpCode::LDARG5"/>
	pub fn load_arg_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 5);
	}
	/// Loads the argument at index 6 onto the evaluation stack.
	/// <see cref="OpCode::LDARG6"/>
	pub fn load_arg_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.load_arg(engine, 6);
	}
	/// Loads the argument at a specified index onto the evaluation stack.
	/// <see cref="OpCode::LDARG"/>
	pub fn load_arg(&self, engine: &mut ExecutionEngine, index: usize) {
		let arguments = match engine.current_context().unwrap().arguments() {
			Some(args) => args,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		let item = match arguments.borrow().get(index) {
			Some(item) => item,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		engine.push(item);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 0.
	/// <see cref="OpCode::STARG0"/>
	pub fn store_arg_0(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 0);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 1.
	/// <see cref="OpCode::STARG1"/>
	pub fn store_arg_1(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 1);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 2.
	/// <see cref="OpCode::STARG2"/>
	pub fn store_arg_2(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 2);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 3.
	/// <see cref="OpCode::STARG3"/>
	pub fn store_arg_3(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 3);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 4.
	/// <see cref="OpCode::STARG4"/>
	pub fn store_arg_4(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 4);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 5.
	/// <see cref="OpCode::STARG5"/>
	pub fn store_arg_5(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 5);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at index 6.
	/// <see cref="OpCode::STARG6"/>
	pub fn store_arg_6(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		self.store_arg(engine, 6);
	}
	/// Stores the value on top of the evaluation stack in the argument slot at a specified index.
	/// <see cref="OpCode::STARG"/>
	pub fn store_arg(&self, engine: &mut ExecutionEngine, index: usize) {
		let item = engine.pop();
		let arguments = match engine.current_context().unwrap().arguments() {
			Some(args) => args,
			None => {
				engine.state = VMState::Fault;
				return;
			},
		};
		if arguments.borrow_mut().set(index, item).is_err() {
			engine.state = VMState::Fault;
		}
	}
}
