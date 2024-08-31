use lazy_static::lazy_static;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
	collections::HashMap,
	fmt::{Display, Error},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
	// Constants
	PUSHINT8 = 0x00,
	PUSHINT16 = 0x01,
	PUSHINT32 = 0x02,
	PUSHINT64 = 0x03,
	PUSHINT128 = 0x04,
	PUSHINT256 = 0x05,
	PUSHT = 0x08,
	PUSHF = 0x09,
	PUSHA = 0x0A,
	PUSHNULL = 0x0B,
	PUSHDATA1 = 0x0C,
	PUSHDATA2 = 0x0D,
	PUSHDATA4 = 0x0E,
	PUSHM1 = 0x0F,
	PUSH0 = 0x10,
	PUSH1 = 0x11,
	PUSH2 = 0x12,
	PUSH3 = 0x13,
	PUSH4 = 0x14,
	PUSH5 = 0x15,
	PUSH6 = 0x16,
	PUSH7 = 0x17,
	PUSH8 = 0x18,
	PUSH9 = 0x19,
	PUSH10 = 0x1A,
	PUSH11 = 0x1B,
	PUSH12 = 0x1C,
	PUSH13 = 0x1D,
	PUSH14 = 0x1E,
	PUSH15 = 0x1F,
	PUSH16 = 0x20,

	// Flow control
	NOP = 0x21,
	JMP = 0x22,
	JMP_L = 0x23,
	JMPIF = 0x24,
	JMPIF_L = 0x25,
	JMPIFNOT = 0x26,
	JMPIFNOT_L = 0x27,
	JMPEQ = 0x28,
	JMPEQ_L = 0x29,
	JMPNE = 0x2A,
	JMPNE_L = 0x2B,
	JMPGT = 0x2C,
	JMPGT_L = 0x2D,
	JMPGE = 0x2E,
	JMPGE_L = 0x2F,
	JMPLT = 0x30,
	JMPLT_L = 0x31,
	JMPLE = 0x32,
	JMPLE_L = 0x33,
	CALL = 0x34,
	CALL_L = 0x35,
	CALLA = 0x36,
	CALLT = 0x37,
	ABORT = 0x38,
	ASSERT = 0x39,
	THROW = 0x3A,
	TRY = 0x3B,
	TRY_L = 0x3C,
	ENDTRY = 0x3D,
	ENDTRY_L = 0x3E,
	ENDFINALLY = 0x3F,
	RET = 0x40,
	SYSCALL = 0x41,

	// Stack
	DEPTH = 0x43,
	DROP = 0x45,
	NIP = 0x46,
	XDROP = 0x48,
	CLEAR = 0x49,
	DUP = 0x4A,
	OVER = 0x4B,
	PICK = 0x4D,
	TUCK = 0x4E,
	SWAP = 0x50,
	ROT = 0x51,
	ROLL = 0x52,
	REVERSE3 = 0x53,
	REVERSE4 = 0x54,
	REVERSEN = 0x55,

	// Slots
	INITSSLOT = 0x56,
	INITSLOT = 0x57,
	LDSFLD0 = 0x58,
	LDSFLD1 = 0x59,
	LDSFLD2 = 0x5A,
	LDSFLD3 = 0x5B,
	LDSFLD4 = 0x5C,
	LDSFLD5 = 0x5D,
	LDSFLD6 = 0x5E,
	LDSFLD = 0x5F,
	STSFLD0 = 0x60,
	STSFLD1 = 0x61,
	STSFLD2 = 0x62,
	STSFLD3 = 0x63,
	STSFLD4 = 0x64,
	STSFLD5 = 0x65,
	STSFLD6 = 0x66,
	STSFLD = 0x67,
	LDLOC0 = 0x68,
	LDLOC1 = 0x69,
	LDLOC2 = 0x6A,
	LDLOC3 = 0x6B,
	LDLOC4 = 0x6C,
	LDLOC5 = 0x6D,
	LDLOC6 = 0x6E,
	LDLOC = 0x6F,
	STLOC0 = 0x70,
	STLOC1 = 0x71,
	STLOC2 = 0x72,
	STLOC3 = 0x73,
	STLOC4 = 0x74,
	STLOC5 = 0x75,
	STLOC6 = 0x76,
	STLOC = 0x77,
	LDARG0 = 0x78,
	LDARG1 = 0x79,
	LDARG2 = 0x7A,
	LDARG3 = 0x7B,
	LDARG4 = 0x7C,
	LDARG5 = 0x7D,
	LDARG6 = 0x7E,
	LDARG = 0x7F,
	STARG0 = 0x80,
	STARG1 = 0x81,
	STARG2 = 0x82,
	STARG3 = 0x83,
	STARG4 = 0x84,
	STARG5 = 0x85,
	STARG6 = 0x86,
	STARG = 0x87,

	// Arithmetic
	NEWBUFFER = 0x88,
	MEMCPY = 0x89,
	CAT = 0x8B,
	SUBSTR = 0x8C,
	LEFT = 0x8D,
	RIGHT = 0x8E,
	INVERT = 0x90,
	AND = 0x91,
	OR = 0x92,
	XOR = 0x93,
	EQUAL = 0x97,
	NOTEQUAL = 0x98,
	SIGN = 0x99,
	ABS = 0x9A,
	NEGATE = 0x9B,
	INC = 0x9C,
	DEC = 0x9D,
	ADD = 0x9E,
	SUB = 0x9F,
	MUL = 0xA0,
	DIV = 0xA1,
	MOD = 0xA2,
	POW = 0xA3,
	SQRT = 0xA4,
	MODMUL = 0xA5,
	MODPOW = 0xA6,
	SHL = 0xA8,
	SHR = 0xA9,
	NOT = 0xAA,
	BOOLAND = 0xAB,
	BOOLOR = 0xAC,
	NZ = 0xB1,
	NUMEQUAL = 0xB3,
	NUMNOTEQUAL = 0xB4,
	LT = 0xB5,
	LE = 0xB6,
	GT = 0xB7,
	GE = 0xB8,
	MIN = 0xB9,
	MAX = 0xBA,
	WITHIN = 0xBB,

	// Compound-type
	PACKMAP = 0xBE,
	PACKSTRUCT = 0xBF,
	PACK = 0xC0,
	UNPACK = 0xC1,
	NEWARRAY0 = 0xC2,
	NEWARRAY = 0xC3,
	NEWARRAY_T = 0xC4,
	NEWSTRUCT0 = 0xC5,
	NEWSTRUCT = 0xC6,
	NEWMAP = 0xC8,
	SIZE = 0xCA,
	HASKEY = 0xCB,
	KEYS = 0xCC,
	VALUES = 0xCD,
	PICKITEM = 0xCE,
	APPEND = 0xCF,
	SETITEM = 0xD0,
	REVERSEITEMS = 0xD1,
	REMOVE = 0xD2,
	CLEARITEMS = 0xD3,
	POPITEM = 0xD4,

	// Types
	ISNULL = 0xD8,
	ISTYPE = 0xD9,
	CONVERT = 0xDB,

	// Exceptions
	ABORTMSG = 0xE0,
	ASSERTMSG = 0xE1,
}

impl OpCode {
	pub fn from_u8(value: u8) -> Option<Self> {
		FromPrimitive::from_u8(value)
	}

	pub fn operand_prefix(&self) -> u8 {
		match self {
			OpCode::PUSHDATA1 => 1,
			OpCode::PUSHDATA2 => 2,
			OpCode::PUSHDATA4 => 4,
			_ => 0,
		}
	}

	pub fn operand_size(&self) -> u8 {
		match self {
			OpCode::PUSHINT8 => 1,
			OpCode::PUSHINT16 => 2,
			OpCode::PUSHINT32 => 4,
			OpCode::PUSHINT64 => 8,
			OpCode::PUSHINT128 => 16,
			OpCode::PUSHINT256 => 32,
			OpCode::PUSHA => 4,
			OpCode::JMP
			| OpCode::JMPIF
			| OpCode::JMPIFNOT
			| OpCode::JMPEQ
			| OpCode::JMPNE
			| OpCode::JMPGT
			| OpCode::JMPGE
			| OpCode::JMPLT
			| OpCode::JMPLE
			| OpCode::CALL => 1,
			OpCode::JMP_L
			| OpCode::JMPIF_L
			| OpCode::JMPIFNOT_L
			| OpCode::JMPEQ_L
			| OpCode::JMPNE_L
			| OpCode::JMPGT_L
			| OpCode::JMPGE_L
			| OpCode::JMPLT_L
			| OpCode::JMPLE_L
			| OpCode::CALL_L => 4,
			OpCode::CALLT => 2,
			OpCode::TRY => 2,
			OpCode::TRY_L => 8,
			OpCode::ENDTRY => 1,
			OpCode::ENDTRY_L => 4,
			OpCode::SYSCALL => 4,
			OpCode::INITSLOT => 2,
			OpCode::LDSFLD
			| OpCode::STSFLD
			| OpCode::LDLOC
			| OpCode::STLOC
			| OpCode::LDARG
			| OpCode::STARG
			| OpCode::NEWARRAY_T
			| OpCode::ISTYPE
			| OpCode::CONVERT => 1,
			_ => 0,
		}
	}
}

struct OperandSize {
	prefix: u8,
	size: u8,
}

lazy_static! {
	static ref OPERAND_SIZE_PREFIX_TABLE: [usize; 256] = {
		let mut table = [0; 256];
		table[OpCode::PUSHDATA1 as usize] = 1;
		table[OpCode::PUSHDATA2 as usize] = 2;
		table[OpCode::PUSHDATA4 as usize] = 4;
		table
	};
	static ref OPERAND_SIZE_TABLE: [usize; 256] = {
		let mut table = [0; 256];

		table[OpCode::PUSHINT8 as usize] = 1;
		table[OpCode::PUSHINT16 as usize] = 2;
		table[OpCode::PUSHINT32 as usize] = 4;
		table[OpCode::PUSHINT64 as usize] = 8;
		table[OpCode::PUSHINT128 as usize] = 16;
		table[OpCode::PUSHINT256 as usize] = 32;
		table[OpCode::PUSHA as usize] = 4;
		table[OpCode::JMP as usize] = 1;
		table[OpCode::JMP_L as usize] = 4;
		table[OpCode::JMPIF as usize] = 1;
		table[OpCode::JMPIF_L as usize] = 4;
		table[OpCode::JMPIFNOT as usize] = 1;
		table[OpCode::JMPIFNOT_L as usize] = 4;
		table[OpCode::JMPEQ as usize] = 1;
		table[OpCode::JMPEQ_L as usize] = 4;
		table[OpCode::JMPNE as usize] = 1;
		table[OpCode::JMPNE_L as usize] = 4;
		table[OpCode::JMPGT as usize] = 1;
		table[OpCode::JMPGT_L as usize] = 4;
		table[OpCode::JMPGE as usize] = 1;
		table[OpCode::JMPGE_L as usize] = 4;
		table[OpCode::JMPLT as usize] = 1;
		table[OpCode::JMPLT_L as usize] = 4;
		table[OpCode::JMPLE as usize] = 1;
		table[OpCode::JMPLE_L as usize] = 4;
		table[OpCode::CALL as usize] = 1;
		table[OpCode::CALL_L as usize] = 4;
		table[OpCode::CALLT as usize] = 2;
		table[OpCode::TRY as usize] = 2;
		table[OpCode::TRY_L as usize] = 8;
		table[OpCode::ENDTRY as usize] = 1;
		table[OpCode::ENDTRY_L as usize] = 4;
		table[OpCode::SYSCALL as usize] = 4;
		table[OpCode::INITSLOT as usize] = 1;
		table[OpCode::INITSLOT as usize] = 2;
		table[OpCode::LDSFLD as usize] = 1;
		table[OpCode::STSFLD as usize] = 1;
		table[OpCode::LDLOC as usize] = 1;
		table[OpCode::STLOC as usize] = 1;
		table[OpCode::LDARG as usize] = 1;
		table[OpCode::STARG as usize] = 1;
		table[OpCode::NEWARRAY_T as usize] = 1;
		table[OpCode::ISTYPE as usize] = 1;
		table[OpCode::CONVERT as usize] = 1;

		table
	};
}

// let opcode_sizes = {
// OpCode::PUSHINT8 => 1,
// OpCode::PUSHINT16 => 2,
// OpCode::PUSHINT32 => 4,
// OpCode::PUSHINT64 => 8,
// OpCode::PUSHINT128 => 16,
// OpCode::PUSHINT256 => 32,
// OpCode::PUSHA => 4,
// OpCode::PUSHDATA1 => 1,
// OpCode::PUSHDATA2 => 2,
// OpCode::PUSHDATA4 => 4,
// OpCode::JMP => 1,
// OpCode::JMP_L => 4,
// OpCode::JMPIF => 1,
// OpCode::JMPIF_L => 4,
// OpCode::JMPIFNOT => 1,
// OpCode::JMPIFNOT_L => 4,
// OpCode::JMPEQ => 1,
// OpCode::JMPEQ_L => 4,
// OpCode::JMPNE => 1,
// OpCode::JMPNE_L => 4,
// OpCode::JMPGT => 1,
// OpCode::JMPGT_L => 4,
// OpCode::JMPGE => 1,
// OpCode::JMPGE_L => 4,
// OpCode::JMPLT => 1,
// OpCode::JMPLT_L => 4,
// OpCode::JMPLE => 1,
// OpCode::JMPLE_L => 4,
// OpCode::CALL => 1,
// OpCode::CALL_L => 4,
// OpCode::CALLT => 2,
// OpCode::TRY => 2,
// OpCode::TRY_L => 8,
// OpCode::ENDTRY => 1,
// OpCode::ENDTRY_L => 4,
// OpCode::XDROP => 1,
// OpCode::PICK => 1,
// OpCode::LDSFLD => 1,
// OpCode::STSFLD => 1,
// OpCode::LDLOC => 1,
// OpCode::STLOC => 1,
// OpCode::LDARG => 1,
// OpCode::STARG => 1,
// OpCode::NEWARRAY_T => 1,
// OpCode::ISTYPE => 1,
// OpCode::CONVERT => 1,
// OpCode::ABORTMSG => 0,
// OpCode::ASSERTMSG => 0,
// };
