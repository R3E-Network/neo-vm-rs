use std::{ collections::HashMap, fmt, hash::Hash
};

use crate::{
	types::{
		stack_item::StackItem, stack_item_type::StackItemType, vm_stack_item::VMStackItem
	},
	vm::execution_engine_limits::ExecutionEngineLimits,
};
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VMBoolean {
	value: bool,
}

impl VMBoolean {
	const TRUE: [u8; 1] = [1];
	const FALSE: [u8; 1] = [0];

	/// Create a new VM object representing the boolean type.
	pub fn new(value: bool) -> Self {
		VMBoolean { value }
	}

	pub fn memory(&self) -> &'static [u8] {
		if self.value {
			&Self::TRUE
		} else {
			&Self::FALSE
		}
	}

	pub fn size(&self) -> usize {
		std::mem::size_of::<bool>()
	}
}

impl StackItem for VMBoolean {
	fn get_type(&self) -> StackItemType {
		StackItemType::Boolean
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		if other.get_type() == StackItemType::Boolean {
			return self.value == other.get_boolean();
		}
		false
	}

	fn get_boolean(&self) -> bool {
		self.value
	}

	fn get_integer(&self) -> BigInt {
		if self.value {
			BigInt::from(1)
		} else {
			BigInt::from(0)
		}
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
	
	fn cleanup(&mut self) {
			todo!()
		}
	
	fn get_slice(&self) -> Vec<u8> {
			todo!()
		}
	
	fn deep_copy(&self, asImmutable:bool) -> Box<VMStackItem> {
			todo!()
		}
	
	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable:bool) -> Box<VMStackItem> {
			todo!()
		}
	
	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
			todo!()
		}
}


impl From<bool> for VMBoolean {
	fn from(value: bool) -> Self {
		VMBoolean::new(value)
	}
}

impl From<VMStackItem> for VMBoolean {
	fn from(value: VMStackItem) -> Self {
		match value {
			VMStackItem::Boolean(b) => b,
			_ => panic!("Invalid conversion"),
		}
	}
}

impl fmt::Display for VMBoolean {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}
