use crate::{
	execution_engine::ExecutionEngine, instruction::Instruction, op_code::OpCode,
	vm::vm_error::VMError, vm_state::VMState,
};
use std::collections::HashMap;

mod bitwise;
mod compound;
mod control;
mod numeric;
mod push;
mod slot;
mod splice;
mod stack;
mod types;

pub struct JumpTable {
	table: HashMap<OpCode, fn(&JumpTable, &mut ExecutionEngine, &Instruction)>,
}

impl JumpTable {
	/// Default JumpTable
	pub const DEFAULT: Self = Self::new();

	pub fn new() -> Self {
		let mut jump_table = Self { table: HashMap::new() };
		jump_table.initialize();
		jump_table
	}
	fn initialize(&mut self) {
		// Push operations
		self.table.insert(OpCode::PUSHINT8, Self::push_int8);
		self.table.insert(OpCode::PUSHINT16, Self::push_int16);
		self.table.insert(OpCode::PUSHINT32, Self::push_int32);
		self.table.insert(OpCode::PUSHINT64, Self::push_int64);
		self.table.insert(OpCode::PUSHINT128, Self::push_int128);
		self.table.insert(OpCode::PUSHINT256, Self::push_int256);
		self.table.insert(OpCode::PUSHT, Self::push_true);
		self.table.insert(OpCode::PUSHF, Self::push_false);
		self.table.insert(OpCode::PUSHA, Self::push_a);
		self.table.insert(OpCode::PUSHNULL, Self::push_null);
		self.table.insert(OpCode::PUSHDATA1, Self::push_data1);
		self.table.insert(OpCode::PUSHDATA2, Self::push_data2);
		self.table.insert(OpCode::PUSHDATA4, Self::push_data4);
		self.table.insert(OpCode::PUSHM1, Self::push_m1);
		self.table.insert(OpCode::PUSH0, Self::push0);
		self.table.insert(OpCode::PUSH1, Self::push1);
		self.table.insert(OpCode::PUSH2, Self::push2);
		self.table.insert(OpCode::PUSH3, Self::push3);
		self.table.insert(OpCode::PUSH4, Self::push4);
		self.table.insert(OpCode::PUSH5, Self::push5);
		self.table.insert(OpCode::PUSH6, Self::push6);
		self.table.insert(OpCode::PUSH7, Self::push7);
		self.table.insert(OpCode::PUSH8, Self::push8);
		self.table.insert(OpCode::PUSH9, Self::push9);
		self.table.insert(OpCode::PUSH10, Self::push10);
		self.table.insert(OpCode::PUSH11, Self::push11);
		self.table.insert(OpCode::PUSH12, Self::push12);
		self.table.insert(OpCode::PUSH13, Self::push13);
		self.table.insert(OpCode::PUSH14, Self::push14);
		self.table.insert(OpCode::PUSH15, Self::push15);
		self.table.insert(OpCode::PUSH16, Self::push16);

		// Control operations
		self.table.insert(OpCode::NOP, Self::nop);
		self.table.insert(OpCode::JMP, Self::jmp);
		self.table.insert(OpCode::JMP_L, Self::jmp_l);
		self.table.insert(OpCode::JMPIF, Self::jmp_if);
		self.table.insert(OpCode::JMPIF_L, Self::jmp_if_l);
		self.table.insert(OpCode::JMPIFNOT, Self::jmp_if_not);
		self.table.insert(OpCode::JMPIFNOT_L, Self::jmp_if_not_l);
		self.table.insert(OpCode::JMPEQ, Self::jmp_eq);
		self.table.insert(OpCode::JMPEQ_L, Self::jmp_eq_l);
		self.table.insert(OpCode::JMPNE, Self::jmp_ne);
		self.table.insert(OpCode::JMPNE_L, Self::jmp_ne_l);
		self.table.insert(OpCode::JMPGT, Self::jmp_gt);
		self.table.insert(OpCode::JMPGT_L, Self::jmp_gt_l);
		self.table.insert(OpCode::JMPGE, Self::jmp_ge);
		self.table.insert(OpCode::JMPGE_L, Self::jmp_ge_l);
		self.table.insert(OpCode::JMPLT, Self::jmp_lt);
		self.table.insert(OpCode::JMPLT_L, Self::jmp_lt_l);
		self.table.insert(OpCode::JMPLE, Self::jmp_le);
		self.table.insert(OpCode::JMPLE_L, Self::jmp_le_l);
		self.table.insert(OpCode::CALL, Self::call);
		self.table.insert(OpCode::CALL_L, Self::call_l);
		self.table.insert(OpCode::CALLA, Self::call_a);
		self.table.insert(OpCode::CALLT, Self::call_t);
		self.table.insert(OpCode::ABORT, Self::abort);
		self.table.insert(OpCode::ASSERT, Self::assert);
		self.table.insert(OpCode::THROW, Self::throw);
		self.table.insert(OpCode::TRY, Self::try_op);
		self.table.insert(OpCode::TRY_L, Self::try_l);
		self.table.insert(OpCode::ENDTRY, Self::end_try);
		self.table.insert(OpCode::ENDTRY_L, Self::end_try_l);
		self.table.insert(OpCode::ENDFINALLY, Self::end_finally);
		self.table.insert(OpCode::RET, Self::ret);
		self.table.insert(OpCode::SYSCALL, Self::syscall);

		// Stack operations
		self.table.insert(OpCode::DEPTH, Self::depth);
		self.table.insert(OpCode::DROP, Self::drop);
		self.table.insert(OpCode::NIP, Self::nip);
		self.table.insert(OpCode::XDROP, Self::xdrop);
		self.table.insert(OpCode::CLEAR, Self::clear);
		self.table.insert(OpCode::DUP, Self::dup);
		self.table.insert(OpCode::OVER, Self::over);
		self.table.insert(OpCode::PICK, Self::pick);
		self.table.insert(OpCode::TUCK, Self::tuck);
		self.table.insert(OpCode::SWAP, Self::swap);
		self.table.insert(OpCode::ROT, Self::rot);
		self.table.insert(OpCode::ROLL, Self::roll);
		self.table.insert(OpCode::REVERSE3, Self::reverse3);
		self.table.insert(OpCode::REVERSE4, Self::reverse4);
		self.table.insert(OpCode::REVERSEN, Self::reverse_n);

		// Slot operations
		self.table.insert(OpCode::INITSLOT, Self::init_slot);
		self.table.insert(OpCode::LDSFLD0, Self::load_static_field_0);
		self.table.insert(OpCode::LDSFLD1, Self::load_static_field_1);
		self.table.insert(OpCode::LDSFLD2, Self::load_static_field_2);
		self.table.insert(OpCode::LDSFLD3, Self::load_static_field_3);
		self.table.insert(OpCode::LDSFLD4, Self::load_static_field_4);
		self.table.insert(OpCode::LDSFLD5, Self::load_static_field_5);
		self.table.insert(OpCode::LDSFLD6, Self::load_static_field_6);
		self.table.insert(OpCode::LDSFLD, Self::load_static_field);
		self.table.insert(OpCode::STSFLD0, Self::store_static_field_0);
		self.table.insert(OpCode::STSFLD1, Self::store_static_field_1);
		self.table.insert(OpCode::STSFLD2, Self::store_static_field_2);
		self.table.insert(OpCode::STSFLD3, Self::store_static_field_3);
		self.table.insert(OpCode::STSFLD4, Self::store_static_field_4);
		self.table.insert(OpCode::STSFLD5, Self::store_static_field_5);
		self.table.insert(OpCode::STSFLD6, Self::store_static_field_6);
		self.table.insert(OpCode::STSFLD, Self::store_static_field);
		self.table.insert(OpCode::LDLOC0, Self::load_local_0);
		self.table.insert(OpCode::LDLOC1, Self::load_local_1);
		self.table.insert(OpCode::LDLOC2, Self::load_local_2);
		self.table.insert(OpCode::LDLOC3, Self::load_local_3);
		self.table.insert(OpCode::LDLOC4, Self::load_local_4);
		self.table.insert(OpCode::LDLOC5, Self::load_local_5);
		self.table.insert(OpCode::LDLOC6, Self::load_local_6);
		self.table.insert(OpCode::LDLOC, Self::load_local);
		self.table.insert(OpCode::STLOC0, Self::store_local_0);
		self.table.insert(OpCode::STLOC1, Self::store_local_1);
		self.table.insert(OpCode::STLOC2, Self::store_local_2);
		self.table.insert(OpCode::STLOC3, Self::store_local_3);
		self.table.insert(OpCode::STLOC4, Self::store_local_4);
		self.table.insert(OpCode::STLOC5, Self::store_local_5);
		self.table.insert(OpCode::STLOC6, Self::store_local_6);
		self.table.insert(OpCode::STLOC, Self::store_local);
		self.table.insert(OpCode::LDARG0, Self::load_arg_0);
		self.table.insert(OpCode::LDARG1, Self::load_arg_1);
		self.table.insert(OpCode::LDARG2, Self::load_arg_2);
		self.table.insert(OpCode::LDARG3, Self::load_arg_3);
		self.table.insert(OpCode::LDARG4, Self::load_arg_4);
		self.table.insert(OpCode::LDARG5, Self::load_arg_5);
		self.table.insert(OpCode::LDARG6, Self::load_arg_6);
		self.table.insert(OpCode::LDARG, Self::load_arg);
		self.table.insert(OpCode::STARG0, Self::store_arg_0);
		self.table.insert(OpCode::STARG1, Self::store_arg_1);
		self.table.insert(OpCode::STARG2, Self::store_arg_2);
		self.table.insert(OpCode::STARG3, Self::store_arg_3);
		self.table.insert(OpCode::STARG4, Self::store_arg_4);
		self.table.insert(OpCode::STARG5, Self::store_arg_5);
		self.table.insert(OpCode::STARG6, Self::store_arg_6);
		self.table.insert(OpCode::STARG, Self::store_arg);

		// Compound-type operations
		self.table.insert(OpCode::NEWARRAY0, Self::new_array0);
		self.table.insert(OpCode::NEWARRAY, Self::new_array);
		self.table.insert(OpCode::NEWARRAYT, Self::new_array_t);
		self.table.insert(OpCode::NEWSTRUCT0, Self::new_struct0);
		self.table.insert(OpCode::NEWSTRUCT, Self::new_struct);
		self.table.insert(OpCode::NEWMAP, Self::new_map);
		self.table.insert(OpCode::SIZE, Self::size);
		self.table.insert(OpCode::KEYS, Self::keys);
		self.table.insert(OpCode::VALUES, Self::values);
		self.table.insert(OpCode::PICKITEM, Self::pick_item);
		self.table.insert(OpCode::APPEND, Self::append);
		self.table.insert(OpCode::SETITEM, Self::set_item);
		self.table.insert(OpCode::REVERSEITEMS, Self::reverse_items);
		self.table.insert(OpCode::REMOVE, Self::remove);
		self.table.insert(OpCode::CLEARITEMS, Self::clear_items);
		self.table.insert(OpCode::POPITEM, Self::pop_item);

		// Numeric operations
		self.table.insert(OpCode::SIGN, Self::sign);
		self.table.insert(OpCode::ABS, Self::abs);
		self.table.insert(OpCode::NEGATE, Self::negate);
		self.table.insert(OpCode::INC, Self::inc);
		self.table.insert(OpCode::DEC, Self::dec);
		self.table.insert(OpCode::ADD, Self::add);
		self.table.insert(OpCode::SUB, Self::sub);
		self.table.insert(OpCode::MUL, Self::mul);
		self.table.insert(OpCode::DIV, Self::div);
		self.table.insert(OpCode::MOD, Self::mod_op);
		self.table.insert(OpCode::POW, Self::pow);
		self.table.insert(OpCode::SQRT, Self::sqrt);
		self.table.insert(OpCode::MODMUL, Self::mod_mul);
		self.table.insert(OpCode::MODPOW, Self::mod_pow);
		self.table.insert(OpCode::SHL, Self::shl);
		self.table.insert(OpCode::SHR, Self::shr);
		self.table.insert(OpCode::NOT, Self::not);
		self.table.insert(OpCode::BOOLAND, Self::bool_and);
		self.table.insert(OpCode::BOOLOR, Self::bool_or);
		self.table.insert(OpCode::NUMEQUAL, Self::num_equal);
		self.table.insert(OpCode::NUMNOTEQUAL, Self::num_not_equal);
		self.table.insert(OpCode::LT, Self::lt);
		self.table.insert(OpCode::LE, Self::le);
		self.table.insert(OpCode::GT, Self::gt);
		self.table.insert(OpCode::GE, Self::ge);
		self.table.insert(OpCode::MIN, Self::min);
		self.table.insert(OpCode::MAX, Self::max);
		self.table.insert(OpCode::WITHIN, Self::within);

		// Bitwise logic operations
		self.table.insert(OpCode::INVERT, Self::invert);
		self.table.insert(OpCode::AND, Self::and);
		self.table.insert(OpCode::OR, Self::or);
		self.table.insert(OpCode::XOR, Self::xor);
		self.table.insert(OpCode::EQUAL, Self::equal);
		self.table.insert(OpCode::NOTEQUAL, Self::not_equal);

		// Splice operations
		self.table.insert(OpCode::NEWBUFFER, Self::new_buffer);
		self.table.insert(OpCode::MEMCPY, Self::memcpy);
		self.table.insert(OpCode::CAT, Self::cat);
		self.table.insert(OpCode::SUBSTR, Self::substr);
		self.table.insert(OpCode::LEFT, Self::left);
		self.table.insert(OpCode::RIGHT, Self::right);

		// Types operations
		self.table.insert(OpCode::ISNULL, Self::is_null);
		self.table.insert(OpCode::ISTYPE, Self::is_type);
		self.table.insert(OpCode::CONVERT, Self::convert);
	}

	pub fn execute(&self, engine: &mut ExecutionEngine, instruction: &Instruction) {
		if let Some(func) = self.table.get(&instruction.opcode) {
			func(self, engine, instruction)
		} else {
			
			Err(VMState::Fault)
		}
	}
}

impl std::ops::Index<OpCode> for JumpTable {
	type Output = fn(&JumpTable, &mut ExecutionEngine, &Instruction) -> Result<(), VMError>;
	fn index(&self, opcode: OpCode) -> &Self::Output {
		self.table
			.get(&opcode)
			.ok_or_else(|| VMError::InvalidOpcode(format!("Invalid opcode: {:?}", opcode)))
			.unwrap()
	}
}
