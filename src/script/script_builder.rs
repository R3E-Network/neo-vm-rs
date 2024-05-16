use std::{error::Error, num::ParseIntError, vec};

use crate::op_code::OpCode;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{pow, FromPrimitive, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash,Serialize,Deserialize)]
pub struct ScriptBuilder {
	output: Vec<u8>,
}

impl ScriptBuilder {
	pub fn new() -> Self {
		Self { output: Vec::new() }
	}

	pub fn len(&self) -> usize {
		self.output.len()
	}

	pub fn emit(&mut self, opcode: OpCode, operand: Vec<u8>) -> &ScriptBuilder {
		self.output.push(opcode as u8);
		self.output.extend(operand);
		return self
	}

	pub fn emit_call(&mut self, offset: i32) -> &ScriptBuilder {
		if offset < i8::MIN.into() || offset > i8::MAX.into() {
			return self.emit(OpCode::CallL, offset.to_le_bytes().to_vec())
		} else {
			return self.emit(OpCode::Call, vec![offset as u8]);
		}
	}

	pub fn emit_jump(&mut self, opcode: OpCode, offset: i32) -> Result<&ScriptBuilder, String> {
		let mut opcode_u8 = opcode as u8;
		if opcode_u8 < (OpCode::Jmp as u8) || opcode_u8 > (OpCode::JmpLeL as u8) {
			return Err(format!("Invalid OpCode {}", opcode_u8));
		}
		if opcode_u8 % 2 == 0 && (offset < i8::MIN.into() || offset > i8::MAX.into()) {
			opcode_u8 += 1;
		}
		if opcode_u8 % 2 == 0 {
			Ok(self.emit(OpCode::from_u8(opcode_u8).unwrap(), vec![offset as u8]))
		} else {
			Ok(self.emit(OpCode::from_u8(opcode_u8).unwrap(), offset.to_le_bytes().to_vec()))
		}
	}

	pub fn emit_int(&mut self, value: BigInt) -> Result<&ScriptBuilder, String> {
		if value == BigInt::from(-1) {
			return Ok(self.emit(OpCode::PushM1, vec![]));
		}
		if value > BigInt::from(-1) && value <= BigInt::from(16) {
			return Ok(self.emit(OpCode::from_u8(OpCode::Push0 as u8 + value.to_signed_bytes_le()[0]).unwrap(), vec![]));
		}
		let bits = value.bits();
		let is_negative = value.sign() == Sign::Minus;
		if bits > 255 && value != -pow(BigInt::from(2), 255) {
			return Err("Only 32 bytes of BigInt allowed".to_string());
		}
		// let mut buffer = [0u8; 32];
		let mut bytes_written = value.to_signed_bytes_le();
		// buffer[..bytes_written.len()].copy_from_slice(&bytes_written);

		let written_len = bytes_written.len();
		if written_len > 32 {return Err("Only 32 bytes of BigInt allowed".to_string());}
		let (opcode, pad_len) = match bytes_written.len() {
			1 => (OpCode::PushInt8, 1),
			2 => (OpCode::PushInt16, 2),
			bytes if bytes <= 4 => (OpCode::PushInt32, 4),
			bytes if bytes <= 8 => (OpCode::PushInt64, 8),
			bytes if bytes <= 16 => (OpCode::PushInt128, 16),
			_ => (OpCode::PushInt256, 32),
		};

		let sign_byte = if is_negative { 0xFF } else { 0x00 };
		let padded = vec![sign_byte; pad_len - written_len];
		bytes_written.extend(padded);
		Ok(self.emit(opcode, bytes_written))
	}

	pub fn emit_bool(&mut self, value: bool) -> &ScriptBuilder {
		if value {return self.emit(OpCode::PushTrue, vec![]);}
		else {return self.emit(OpCode::PushFalse, vec![]);}
	}

	pub fn emit_bytes(&mut self, data: Vec<u8>) -> &ScriptBuilder {
		match data.len() {
			len if len < 0x100 => {
				self.output.push(OpCode::PushData1 as u8);
				self.output.push(len as u8);
				self.output.extend_from_slice(&data);
			},
			len if len < 0x10000 => {
				self.output.push(OpCode::PushData2 as u8);
				self.output.extend_from_slice(&(len as u16).to_le_bytes());
				self.output.extend_from_slice(&data);
			},
			len => {
				self.output.push(OpCode::PushData4 as u8);
				self.output.extend_from_slice(&(len as u32).to_le_bytes());
				self.output.extend_from_slice(&data);
			},
		}
		return self;
	}

	// Other push methods
	pub fn emit_string(&mut self, data: &str) -> &ScriptBuilder {
		let bytes = data.as_bytes().to_vec();
		self.emit_bytes(bytes);
		return self;
	}

	pub fn emit_raw(&mut self, script: Vec<u8>) -> &ScriptBuilder {
		self.output.extend(script);
		return self;
	}

	pub fn emit_syscall(&mut self, api: u32) {
		let opcode = OpCode::Syscall;
		let operand = api.to_le_bytes().to_vec();
		self.emit(opcode, operand);
	}

	pub fn to_bytes(self) -> Vec<u8> {
		self.output
	}
}

#[cfg(test)]
mod tests {
	use crate::script;

use super::*;

	#[test]
	fn test_emit() {
		let mut script = ScriptBuilder::new();
		assert_eq!(script.len(), 0);
		script.emit(OpCode::Nop, vec![]);
		assert_eq!(script.len(), 1);
		assert_eq!(vec![0x21], script.to_bytes());

		script = ScriptBuilder::new();
		script.emit(OpCode::Nop, vec![0x66]);
		assert_eq!(vec![0x21, 0x66], script.to_bytes())
	}

	#[test]
	fn test_bigint() {
		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(-100000));
		assert_eq!(script.len(), 5);
		assert_eq!(vec![2, 96, 121, 254, 255], script.to_bytes());

		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(100000));
		assert_eq!(script.len(), 5);
		assert_eq!(vec![2, 160, 134, 1, 0], script.to_bytes());
		
		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(32));
		assert_eq!(script.len(), 2);
		assert_eq!(vec![0, 32], script.to_bytes());

		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(-32));
		assert_eq!(script.len(), 2);
		assert_eq!(vec![0, 224], script.to_bytes());  // 224 == 256 - 32

		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(-pow(BigInt::from(2), 255));
		assert_eq!(script.len(), 33);
		assert_eq!(vec![5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80], script.to_bytes());  // 224 == 256 - 32

		let mut script = ScriptBuilder::new();
		let err = script.emit_int(pow(BigInt::from(2), 255)).unwrap_err();
		assert_eq!(err, "Only 32 bytes of BigInt allowed");
	}

	#[test]
	fn test_emit_syscall() {
		let mut script = ScriptBuilder::new();
		script.emit_syscall(0xE393C875u32);
		assert_eq!(vec![OpCode::Syscall as u8, 0x75, 0xC8, 0x93, 0xE3], script.to_bytes());
	}

	#[test]
	fn test_emit_call() {
		let mut script = ScriptBuilder::new();
		script.emit_call(0);
		assert_eq!(vec![OpCode::Call as u8, 0], script.to_bytes());

		let mut script = ScriptBuilder::new();
		script.emit_call(12345);
		assert_eq!(vec![OpCode::CallL as u8, 57, 48, 0, 0], script.to_bytes());

		let mut script = ScriptBuilder::new();
		script.emit_call(-12345);
		assert_eq!(vec![OpCode::CallL as u8, 199, 207, 255, 255], script.to_bytes());
	}

	#[test]
	fn test_emit_jump() {
		// Hard to iterate all values of enum. Skip for now.
	}

	fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
		if s.len() % 2 == 0 {
			(0..s.len())
				.step_by(2)
				.map(|i| s.get(i..i + 2)
						  .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
				.collect()
		} else {
			None
		}
	}

	#[test]
	fn test_emit_push_bigint() {
		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(-1));
		assert_eq!(vec![0x0f], script.to_bytes());

		let mut script = ScriptBuilder::new();
		let _ = script.emit_int(BigInt::from(0));
		assert_eq!(vec![0x10], script.to_bytes());

		for x in 1..17 {
			let mut script = ScriptBuilder::new();
			let _ = script.emit_int(BigInt::from(x));
			assert_eq!(vec![OpCode::Push0 as u8 + x], script.to_bytes());
		}

		assert_eq!(hex_to_bytes("0080").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i8::MIN)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("007f").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i8::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("01ff00").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(u8::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("010080").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i16::MIN)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("01ff7f").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i16::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("02ffff0000").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(u16::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("0200000080").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i32::MIN)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("02ffffff7f").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i32::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("03ffffffff00000000").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(u32::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("030000000000000080").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i64::MIN)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("03ffffffffffffff7f").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(i64::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("04ffffffffffffffff0000000000000000").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(u64::MAX)).unwrap().to_owned().to_bytes());
		assert_eq!(hex_to_bytes("050100000000000000feffffffffffffff00000000000000000000000000000000").unwrap(), ScriptBuilder::new().emit_int(BigInt::from(u64::MAX) * BigInt::from(u64::MAX)).unwrap().to_owned().to_bytes());

		let large_num = BigInt::from_signed_bytes_le(&hex_to_bytes("050100000000000000feffffffffffffff0100000000000000feffffffffffffff00000000000000000000000000000000").unwrap()[..]);
		let err = ScriptBuilder::new().emit_int(large_num).unwrap_err();
		assert_eq!(err, "Only 32 bytes of BigInt allowed");
	}

	#[test]
	fn test_emit_push_bool() {
		assert_eq!(ScriptBuilder::new().emit_bool(true).to_owned().to_bytes(), vec![OpCode::PushTrue as u8]);
		assert_eq!(ScriptBuilder::new().emit_bool(false).to_owned().to_bytes(), vec![OpCode::PushFalse as u8]);
	}

	#[test]
	fn test_emit_push_bytes() {
		let data = vec![0x01, 0x02];
		let len = data.len().try_into().unwrap();
		assert_eq!(ScriptBuilder::new().emit_bytes(data).to_owned().to_bytes(), vec![OpCode::PushData1 as u8, len, 0x01, 0x02]);

		// TODO: more tests
	}
}
