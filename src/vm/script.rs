use std::{collections::HashMap, rc::Rc};

use crate::types::stack_item_type::StackItemType;

use super::{instruction::Instruction, op_code::OpCode};

#[derive(Clone)]
pub struct Script {
	value: Vec<u8>,
	strict_mode: bool,
	instructions: HashMap<usize, Rc<Instruction>>,
}

impl Script {
	pub fn new(script: Vec<u8>) -> Self {
		Self::new_with_mode(script, false)
	}

	pub fn new_with_mode(script: Vec<u8>, strict_mode: bool) -> Self {
		let mut s = Script { value: script.into(), strict_mode, instructions: HashMap::new() };

		if strict_mode {
			s.validate_script().expect("Invalid script");
		}

		s
	}

	pub fn len(&self) -> usize {
		self.value.len()
	}

	pub fn is_empty(&self) -> bool {
		self.value.is_empty()
	}

	pub fn get(&self, index: usize) -> Option<OpCode> {
		self.value.get(index).map(|&b| OpCode::from_u8(b))
	}

	pub fn get_instruction(&mut self, ip: usize) -> Result<Rc<Instruction>, ScriptError> {
		if ip >= self.len() {
			return Err(ScriptError::InvalidInstructionPointer(ip));
		}

		if let Some(instruction) = self.instructions.get(&ip) {
			return Ok(Rc::clone(instruction));
		}

		if self.strict_mode {
			return Err(ScriptError::InstructionNotFound(ip));
		}

		let instruction = Instruction::new(self.value.clone(), ip).map_err(|e| ScriptError::InvalidInstructionPointer(ip))?;
		self.instructions.insert(ip, Rc::new(instruction));
		Ok(Rc::clone(&self.instructions[&ip]))
	}

	fn validate_script(&mut self) -> Result<(), ScriptError> {
		let mut ip = 0;
		while ip < self.len() {
			let instruction = self.get_instruction(ip)?;

			match instruction.opcode {
				OpCode::JMP
				| OpCode::JMPIF
				| OpCode::JMPIFNOT
				| OpCode::JMPEQ
				| OpCode::JMPNE
				| OpCode::JMPGT
				| OpCode::JMPGE
				| OpCode::JMPLT
				| OpCode::JMPLE
				| OpCode::CALL
				| OpCode::ENDTRY => {
					let target = (ip as i32 + instruction.token_i8() as i32) as usize;
					self.get_instruction(target)?;
				},
				OpCode::PUSHA
				| OpCode::JMP_L
				| OpCode::JMPIF_L
				| OpCode::JMPIFNOT_L
				| OpCode::JMPEQ_L
				| OpCode::JMPNE_L
				| OpCode::JMPGT_L
				| OpCode::JMPGE_L
				| OpCode::JMPLT_L
				| OpCode::JMPLE_L
				| OpCode::CALL_L
				| OpCode::ENDTRY_L => {
					let target = (ip as i32 + instruction.token_i32()) as usize;
					self.get_instruction(target)?;
				},
				OpCode::TRY => {
					let catch_target = (ip as i32 + instruction.token_i8() as i32) as usize;
					let finally_target = (ip as i32 + instruction.token_i8_1() as i32) as usize;
					self.get_instruction(catch_target)?;
					self.get_instruction(finally_target)?;
				},
				OpCode::TRY_L => {
					let catch_target = (ip as i32 + instruction.token_i32()) as usize;
					let finally_target = (ip as i32 + instruction.token_i32_1()) as usize;
					self.get_instruction(catch_target)?;
					self.get_instruction(finally_target)?;
				},
				OpCode::NEWARRAY_T | OpCode::ISTYPE | OpCode::CONVERT => {
					let type_code = instruction.token_u8();
					if !StackItemType::is_valid(type_code) {
						return Err(ScriptError::InvalidStackItemType(ip, type_code));
					}
					if instruction.opcode != OpCode::NEWARRAY_T
						&& StackItemType::from(type_code) == StackItemType::Any
					{
						return Err(ScriptError::InvalidStackItemType(ip, type_code));
					}
				},
				_ => {},
			}

			ip += instruction.size();
		}

		Ok(())
	}
}

impl AsRef<[u8]> for Script {
	fn as_ref(&self) -> &[u8] {
		&self.value
	}
}

#[derive(Debug)]
pub enum ScriptError {
	InvalidInstructionPointer(usize),
	InstructionNotFound(usize),
	InvalidJumpTarget(usize, OpCode),
	InvalidTryTarget(usize),
	InvalidStackItemType(usize, u8),

}
