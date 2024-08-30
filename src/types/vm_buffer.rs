use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::vm_error::VMError;
use crate::{
    types::compound_types::compound_type::CompoundType,
};
use num_bigint::{BigInt, Sign};
use crate::vm::execution_engine_limits::ExecutionEngineLimits;
use crate::types::primitive_types::primitive_type::PrimitiveType;
use crate::types::primitive_types::vm_boolean::VMBoolean;
use crate::types::primitive_types::vm_byte_string::VMByteString;
use crate::types::stack_item::{ObjectReferenceEntry, StackItem};
use crate::types::stack_item_type::StackItemType;

use super::compound_types::vm_compound::VMCompound;
use super::primitive_types::vm_primitive::VMPrimitive;
use super::vm_stack_item::VMStackItem;

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct VMBuffer {
	bytes: Cow<'static, [u8]>,
}

impl VMBuffer {
	pub fn new(size: usize) -> Self {
		Self {
			bytes: Cow::Owned(Vec::with_capacity(size)),
		}
	}

	pub fn from_slice(data: &[u8]) -> Self {
		Self {
			bytes: Cow::Borrowed(data),
		}
	}

	fn to_vec(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}

	fn as_slice(&self) -> &[u8] {
		self.bytes.as_ref()
	}
}

impl Drop for VMBuffer {
	fn drop(&mut self) {
		// Return buffer to pool if not static
	}
}

impl StackItem for VMBuffer {

	fn cleanup(&mut self) {
		todo!()
	}

	fn get_slice(&self) -> Vec<u8> {
		self.as_slice().to_vec()	
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Buffer
	}

	fn get_boolean(&self) -> bool {
		true
	}
	fn deep_copy(
		&self,
		_ref_map: &HashMap<&VMStackItem, Box<VMStackItem>>,
		as_immutable: bool,
	) -> Box<VMStackItem> {
		if as_immutable {
			VMByteString::from(self.to_vec()).into()
		} else {
			VMBuffer::from_slice(self.as_slice()).into()
		}
	}
	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		todo!()
	}

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl PrimitiveType for VMBuffer {
	fn memory(&self) -> Vec<u8> {
		self.as_slice().to_vec()
	}

	fn convert_to(&self, ty: StackItemType) -> Result<VMStackItem, VMError> {
		match ty {
			StackItemType::Integer => {
				if self.bytes.len() > i32::MAX as usize {
					panic!("Invalid cast");
				}
				BigInt::from_bytes_le(Sign::NoSign, self.as_slice()).into()
			},
			StackItemType::ByteString => self.to_vec().into(),
			StackItemType::Buffer => VMBuffer::from(self.memory()).into(),
			StackItemType::Boolean => VMBoolean::from(self.get_boolean()).into(),
			_ => panic!("Invalid cast"),
		}
	}
}

impl From<Vec<u8>> for VMBuffer {
	fn from(bytes: Vec<u8>) -> Self {
		Self {
			bytes: Cow::Owned(bytes),
		}
	}
}

impl From<&[u8]> for VMBuffer {
	fn from(bytes: &[u8]) -> Self {
		Self {
			bytes: Cow::Borrowed(bytes),
		}
	}
}