use crate::{
	compound_type::CompoundType,
	primitive_type::PrimitiveType,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
use std::{
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
};

/// Represents `null` in the vm.
pub struct Null;

impl Debug for Null {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Hash for Null {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}

impl Eq for Null {}

impl PartialEq<Self> for Null {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl StackItemTrait for Null {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>;

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

	fn set_object_references(&mut self, refs: Self::ObjectReferences) {
		self.object_references = refs;
	}

	fn object_references(&self) -> &Self::ObjectReferences {
		&self.object_references
	}

	fn set_stack_references(&mut self, count: usize) {
		self.stack_references = count as u32;
	}

	fn stack_references(&self) -> usize {
		self.stack_references as usize
	}

	fn is_null(&self) -> bool {
		true
	}

	fn cleanup(&mut self) {
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> Result<StackItem, Err()> {
		if ty == StackItemType::Any {
			Ok(StackItem::Null(Self))
		} else {
			Err(())
		}
	}

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>, as_immutable: bool) -> StackItem {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		false
	}

	fn get_interface<T: 'static>(&self) -> Result<&T, ()> {
		Err(())
	}

	fn get_slice(&self) -> &[u8] {
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
}

impl Into<StackItem> for Null {
	fn into(self) -> StackItem {
		StackItem::Null(self)
	}
}

impl From<StackItem> for Null {
	fn from(item: StackItem) -> Self {
		match item {
			StackItem::Null(n) => n,
			_ => panic!("Cannot convert {:?} to Null", item),
		}
	}
}