use std::{cell::RefCell, collections::HashMap, hash::Hash};
use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
	vm::script::Script,
};
use crate::execution_engine_limits::ExecutionEngineLimits;
use crate::types::compound_types::compound_type::CompoundType;

use super::compound_types::vm_compound::VMCompound;
use super::stack_item::StackItem;
use super::stack_item_type::StackItemType;
use super::vm_stack_item::VMStackItem;

#[derive(Clone, Hash, Debug)]
pub struct VMPointer {
	script: Script,
	position: usize,
}

impl VMPointer {
	pub fn new(script: &Script, position: usize) -> Self {
		Self {
	
			script:script.clone(),
			position,
		}
	}

	pub fn script(&self) -> &Script {
		&self.script
	}

	pub fn position(&self) -> usize {
		self.position
	}
}



impl StackItem for VMPointer {
	
	fn cleanup(&mut self) {
		todo!()
	}

	fn get_slice(&self) -> Vec<u8> {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Pointer
	}
	fn get_boolean(&self) -> bool {
		true
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		if std::ptr::eq(self, other) {
			return true;
		}
		if let Some(p) = other.as_any().downcast_ref::<VMPointer>() {
			return self.position == p.position && self.script == p.script;
		}
		false
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
