use crate::types::{compound_types::compound_type::CompoundType, primitive_types::vm_primitive::VMPrimitive};
use std::{
	cell::{Ref, RefCell},
	collections::{
		hash_map::{Entry, Iter, IterMut},
		HashMap,
	},
	fmt::Debug,
	hash::Hash,
	rc::Rc,
};
use std::any::Any;
use std::cmp::PartialEq;
use num_bigint::BigInt;
use crate::execution_engine_limits::ExecutionEngineLimits;
use crate::types::primitive_types::primitive_type::PrimitiveType;
use crate::vm::reference_counter::ReferenceCounter;
use crate::types::stack_item::{ObjectReferenceEntry, StackItem};
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_stack_item::VMStackItem;

use super::vm_compound::VMCompound;

#[derive(Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct VMMap {
	stack_references: u32,
	reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	object_references: RefCell<Option<HashMap<VMCompound, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	dictionary: HashMap<Rc<RefCell<VMPrimitive>>, Rc<RefCell<VMStackItem>>>,
	read_only: bool,
}

impl VMMap {
	pub const MAX_KEY_SIZE: usize = 64;

	pub fn new(reference_counter: Option<Rc<RefCell<ReferenceCounter>>>) -> Self {
		Self {
			stack_references: 0,
			reference_counter,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			dictionary: HashMap::new(),
			read_only: false,
		}
	}

	pub fn insert(&mut self, key: Rc<RefCell<VMPrimitive>>, value: Rc<RefCell<VMStackItem>>) {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.insert(key.clone(), value);
	}

	pub fn get(&self, key: Rc<RefCell<VMPrimitive>>) -> Option<Rc<RefCell<VMStackItem>>> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}
		match self.dictionary.get(&key) {
			Some(value) => Some(value.clone()),
			None => None,
		}
	}

	pub fn contains_key(&self, key: Rc<RefCell<VMPrimitive>>) -> bool {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.contains_key(&key)
	}

	pub fn remove(&mut self, key: Rc<RefCell<VMPrimitive>>) -> Option<Rc<RefCell<VMStackItem>>> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.remove(&key)
	}

	pub fn len(&self) -> usize {
		self.dictionary.len()
	}

	pub fn is_empty(&self) -> bool {
		self.dictionary.is_empty()
	}

	pub fn clear(&mut self) {
		self.dictionary.clear();
	}

	pub fn keys(&self) -> Vec<Rc<RefCell<VMStackItem>>> {
		self.dictionary.into_keys().collect()
	}

	pub fn values(&self) -> Vec<Rc<RefCell<VMStackItem>>> {
		self.dictionary.into_values().collect()
	}

	pub fn iter(&self) -> Iter<'_, Rc<RefCell<VMPrimitive>>, Rc<RefCell<VMStackItem>>> {
		self.dictionary.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Rc<RefCell<VMPrimitive>>, Rc<RefCell<VMStackItem>>> {
		self.dictionary.iter_mut()
	}

	pub fn entry(
		&mut self,
		key: Rc<RefCell<VMPrimitive>>,
	) -> Entry<'_, Rc<RefCell<VMPrimitive>>, Rc<RefCell<VMStackItem>>> {
		self.dictionary.entry(key)
	}
}

impl PartialEq for VMStackItem {
	fn eq(&self, other: &Self) -> bool {
		self.equals(other)
	}
}

impl StackItem for VMMap {

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

	fn convert_to(&self, ty: StackItemType) -> Box<VMStackItem> {
		todo!()
	}

	fn get_slice(&self) -> Vec<u8> {
		panic!("Cannot get slice of map")
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Map
	}
	fn get_boolean(&self) -> bool {
		true
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		let mut new_map = VMMap::new(self.reference_counter.clone());
		for (key, value) in self.dictionary.iter() {
			let new_key = key.deep_copy_with_ref_map(ref_map, asImmutable);
			let new_value = value.deep_copy_with_ref_map(ref_map, asImmutable);
			new_map.insert(new_key, new_value);
		}
		Box::new(new_map)
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		if let Some(other_map) = other.get_interface::<VMMap>() {
			self.dictionary == other_map.dictionary
		} else {
			false
		}
	}

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
		if let Some(other_map) = other.get_interface::<VMMap>() {
			self.dictionary == other_map.dictionary
		} else {
			false
		}
	}

	fn get_integer(&self) -> BigInt {
		panic!("Cannot get integer of map")
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		panic!("Cannot get interface of map")
	}

	fn get_bytes(&self) -> &[u8] {
		panic!("Cannot get bytes of map")
	}
}

impl CompoundType for VMMap {
	fn count(&self) -> usize {
		self.dictionary.len()
	}

	fn sub_items(&self) -> Vec<Ref<RefCell<VMStackItem>>> {
		self.dictionary.keys().chain(self.dictionary.values()).cloned().collect()
	}

	fn sub_items_count(&self) -> usize {
		self.count() * 2
	}

	fn read_only(&mut self) {
		self.read_only = true;
	}

	fn is_read_only(&self) -> bool {
		self.read_only
	}

	fn clear(&mut self) {
		if self.read_only {
			panic!("Cannot clear read-only map")
		}
		if self.reference_counter.is_some() {
			for (key, value) in self.dictionary.iter() {
				self.reference_counter
					.unwrap()
					.get_mut()
					.remove_stack_reference(key.clone().into());
				self.reference_counter.unwrap().get_mut().remove_stack_reference(value.clone());
			}
		}
		self.dictionary.clear();
	}
}

impl PartialEq for VMMap {
	fn eq(&self, other: &Self) -> bool {
		self.dictionary == other.dictionary
	}
}
impl Clone for VMMap {
	fn clone(&self) -> Self {
		let mut result = Self::new(self.reference_counter.clone());
		// ref_map.insert(self, result.clone());
		for (key, value) in self.dictionary.iter() {
			result.dictionary.insert(key.clone(), value.clone());
		}

		result
	}
}
