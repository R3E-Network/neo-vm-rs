use std::{
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
};
use num_bigint::BigInt;
use crate::{execution_engine_limits::ExecutionEngineLimits, vm_error::VMError};

use super::{stack_item::StackItem, stack_item_type::StackItemType, vm_stack_item::VMStackItem};

/// Represents `null` in the vm.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct VMNull {
}

impl StackItem for VMNull {

	fn is_null(&self) -> bool {
		true
	}

	fn cleanup(&mut self) {
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> Result<VMStackItem, VMError> {
		if ty == StackItemType::Any {
			Ok(VMNull::into())	
		} else {
			Err(VMError::new("Cannot convert to the given type"))
		}
	}

	fn get_slice(&self) -> Vec<u8> {
		todo!()
	}

	fn get_string(&self) -> Option<String> {
		None
	}

	fn get_hash_code(&self) -> u64 {
		0
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Any
	}

	fn get_boolean(&self) -> bool {
		false
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}
	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		 other.get_type() == StackItemType::Any
	}

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_interface<T: 'static>(&self) -> Option<&T> {
		None
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}

}

impl Into<VMStackItem> for VMNull {
	fn into(self) -> VMStackItem{
		VMStackItem::Null(self)
	}
}

impl From<VMStackItem> for VMNull {
	fn from(item: VMStackItem) -> Self {
		match item {
			VMStackItem::Null(_) => VMNull::default(),
			_ => panic!("Cannot convert {:?} to Null", item),
		}
	}
}
