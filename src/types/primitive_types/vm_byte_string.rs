use std::{cell::RefCell, collections::HashMap, convert::TryInto, hash::Hash, io::Cursor};
use std::any::Any;
use std::hash::Hasher;
use std::rc::Rc;

use crate::types::compound_types::vm_compound::VMCompound;
use crate::{
    types::{
		compound_types::compound_type::CompoundType,
		primitive_types::primitive_type::{PrimitiveType},
	},
};
use murmur3::murmur3_32;
use num_bigint::BigInt;
use crate::vm::execution_engine_limits::ExecutionEngineLimits;
use crate::types::stack_item::{ObjectReferenceEntry, StackItem};
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_stack_item::VMStackItem;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct VMByteString {
	bytes: Vec<u8>,
	hash: u32,
}

impl VMByteString {
	pub const EMPTY: Self = Self {
		bytes: Vec::new(),
		hash: 0,
	};

	pub fn new(bytes: Vec<u8>) -> Self {
		Self {
			bytes,
			hash: 0,
		}
	}

	fn equals(&self, other: &Self) -> bool {
		self.bytes == other.bytes
	}

	fn hash(&mut self) -> u32 {
		self.hash
			.unwrap_or_else(|| murmur3_32(&mut Cursor::new(&self.bytes), 0).unwrap())
	}
}


impl PrimitiveType for VMByteString {
	fn memory(&self) -> &[u8] {
		self.get_slice()
	}
}

impl PartialEq<VMStackItem> for VMByteString {
	fn eq(&self, other: &Self) -> bool {
		self.equals(other)
	}
}


impl StackItem for VMByteString {

	fn cleanup(&mut self) {
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> Box<VMStackItem> {
		todo!()
	}


	fn get_slice(&self) -> Vec<u8> {
		self.bytes.as_slice()
	}

	fn get_hash_code(&self) -> u64 {
		if self.hash == 0 {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			hasher.write(&self.bytes);
			self.hash = hasher.finish() as u32;
		}
		self.hash as u64
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::ByteString
	}

	fn get_boolean(&self) -> bool {
		self.bytes.iter().all(|&x| x == 0x00)
	}

	fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		todo!()
	}

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
		if self.bytes.len() > limits.max_comparable_size || other.get_slice().len() > limits.max_comparable_size {
			panic!("Max comparable size exceeded")
		} else {
			self.equals(other)
		}
	}

	fn from_interface(value: &dyn Any) -> Box<VMStackItem> {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}
	fn get_bytes(&self) -> &[u8] {
		&self.bytes
	}
}