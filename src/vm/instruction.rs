use crate::op_code::OpCode;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Instruction {
	pub opcode: OpCode,
	pub operand: Vec<u8>,
}

#[derive(Debug)]
enum Error {
	InvalidOpcode,
	InvalidOperandSize,
	InvalidPrefixSize(usize),
	OperandOutOfBounds { instruction_pointer: usize, operand_size: usize, script_length: usize },
}

impl Instruction {
	pub const RET: Self = Self { opcode: OpCode::RET, operand: Vec::new() };

	pub fn new(script: Vec<u8>, ip: usize) -> Result<Self, Error> {
		if ip >= script.len() {
			return Err(Error::InvalidOperandSize);
		}
		
		let opcode = OpCode::from_u8(script[ip]).unwrap();
		
		let operand_size = opcode.operand_size() as usize;
		let prefix_size = opcode.operand_prefix() as usize;
		
		if prefix_size > 0 {
			if ip + 1 + prefix_size > script.len() {
				return Err(Error::InvalidPrefixSize(prefix_size));
			}
			let operand_size = match prefix_size {
				1 => script[ip + 1] as usize,
				2 => u16::from_le_bytes([script[ip + 1], script[ip + 2]]).into(),
				4 => u32::from_le_bytes([script[ip + 1], script[ip + 2], script[ip + 3], script[ip + 4]]).try_into().unwrap(),
				_ => return Err(Error::InvalidPrefixSize(prefix_size)),
			};
			if ip + 1 + prefix_size + operand_size > script.len() {
				return Err(Error::OperandOutOfBounds {
					instruction_pointer: ip,
					operand_size,
					script_length: script.len(),
				});
			}
			let operand = script[ip + 1 + prefix_size..ip + 1 + prefix_size + operand_size].to_vec();
			Ok(Self { opcode, operand })
		} else {
			if ip + 1 + operand_size > script.len() {
				return Err(Error::OperandOutOfBounds {
					instruction_pointer: ip,
					operand_size,
					script_length: script.len(),
				});
			}
			let operand = script[ip + 1..ip + 1 + operand_size].to_vec();
			Ok(Self { opcode, operand })
		}
	}


	pub fn size(&self) -> usize {
		let prefix_size = self.opcode.operand_prefix(); //  OPERAND_SIZE_PREFIX[self.opcode as usize];
		if prefix_size > 0 {
			(1 + prefix_size + self.operand.len() as u8) as usize
		} else {
			(1 + self.opcode.operand_size()) as usize
		}
	}

	// Token getters
	pub fn token_i8(&self) -> i8 {
		self.operand[0] as i8
	}

	pub fn token_i8_1(&self) -> i8 {
		self.operand[1] as i8
	}

	pub fn token_i32(&self) -> i32 {
		i32::from_le_bytes(self.operand[..4].try_into().unwrap())
	}

	pub fn token_i32_1(&self) -> i32 {
		i32::from_le_bytes(self.operand[4..8].try_into().unwrap())
	}

	// Other token methods
	pub fn token_u8(&self) -> u8 {
		self.operand[0]
	}

	pub fn token_u8_1(&self) -> u8 {
		self.operand[1]
	}

	pub fn token_u16(&self) -> u16 {
		u16::from_le_bytes(self.operand[..2].try_into().unwrap())
	}

	pub fn token_u32(&self) -> u32 {
		u32::from_le_bytes(self.operand[..4].try_into().unwrap())
	}

	pub fn token_i256(&self) -> [u8; 32] {
		let mut result = [0u8; 32];
		result.copy_from_slice(&self.operand[..32]);
		result
	}

	pub fn token_string(&self) -> String {
		String::from_utf8(self.operand.clone()).unwrap()
	}
	pub fn from_script(script: &[u8], ip: usize) -> Result<Self, Error> {
		let opcode = OpCode::from_u8(script[ip]).unwrap();
		let mut ip = ip + 1;

		let mut operand_size = 0;
		let prefix_size = opcode.operand_prefix() as usize;
		match prefix_size {
			0 => {
				operand_size = opcode.operand_size() as usize;
			},
			1 => {
				operand_size = script[ip] as usize;
				ip += 1;
			},
			2 => {
				operand_size = u16::from_le_bytes([script[ip], script[ip + 1]]) as usize;
				ip += 2;
			},
			4 => {
				operand_size = i32::from_le_bytes([
					script[ip],
					script[ip + 1],
					script[ip + 2],
					script[ip + 3],
				]) as usize;
				ip += 4;
			},
			_ => return Err(Error::InvalidPrefixSize(prefix_size)),
		}

		let operand = script[ip..ip + operand_size].to_vec();
		Ok(Self { opcode, operand })
	}
}

impl std::fmt::Display for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.opcode)
	}
}
