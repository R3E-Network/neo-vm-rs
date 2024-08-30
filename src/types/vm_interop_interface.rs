use std::{
	any::{Any, TypeId},
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
};

use crate::vm_error::VMError;

use super::{compound_types::{compound_type::CompoundType, vm_compound::VMCompound}, stack_item::{ObjectReferenceEntry, StackItem}, stack_item_type::StackItemType, vm_stack_item::VMStackItem};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VMInteropInterface {
	stack_references: u32,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	object: Box<dyn Any>,
}

impl VMInteropInterface {

}

impl StackItem for VMInteropInterface {

	fn dfn(&self) -> isize {
		self.dfn
	}

	fn set_dfn(&mut self, dfn: isize) {
		self.dfn = dfn;
	}

	fn low_link(&self) -> usize {
		self.low_link
	}

	fn set_low_link(&mut self, link: usize) {
		self.low_link = link;
	}

	fn on_stack(&self) -> bool {
		self.on_stack
	}

	fn set_on_stack(&mut self, on_stack: bool) {
		self.on_stack = on_stack;
	}

	fn set_object_references(&mut self, refs: RefCell<HashMap<VMCompound, ObjectReferenceEntry>>) {
		self.object_references = refs;
	}

	fn object_references(&self) -> RefCell<HashMap<VMCompound, ObjectReferenceEntry>> {
		self.object_references
	}

	fn set_stack_references(&mut self, count: usize) {
		self.stack_references = count as u32;
	}

	fn stack_references(&self) -> usize {
		self.stack_references as usize
	}

	fn cleanup(&mut self) {
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> Result<VMStackItem, VMError> {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		true
	}

	fn get_interface<T: Any>(&self) -> Result<&T, VMError> {
		self.object
			.downcast_ref::<T>()
			.ok_or(InvalidCastError(format!("Cannot cast to {}", std::any::type_name::<T>())))
	}

	fn get_slice(&self) -> Vec<u8> {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::InteropInterface
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		match other {
			Some(o) => {
				if self == o.as_ref() {
					return true
				}
				if let Some(i) = o.downcast_ref::<VMInteropInterface>() {
					self.object.eq(&i.object)
				} else {
					false
				}
			},
			None => false,
		}
	}
	
	fn deep_copy(&self, asImmutable:bool) -> Box<super::vm_stack_item::VMStackItem> {
			todo!()
		}
	
	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&super::vm_stack_item::VMStackItem, &super::vm_stack_item::VMStackItem>, asImmutable:bool) -> Box<super::vm_stack_item::VMStackItem> {
			todo!()
		}
	
	fn equals_with_limits(&self, other: &super::vm_stack_item::VMStackItem, limits: &crate::execution_engine_limits::ExecutionEngineLimits) -> bool {
			todo!()
		}
	
	fn get_integer(&self) -> num_bigint::BigInt {
			todo!()
		}
	
	fn get_bytes(&self) -> &[u8] {
			todo!()
		}
}
