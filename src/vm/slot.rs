use crate::{null::Null, reference_counter::ReferenceCounter, stack_item::StackItem};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Slot {
	items: Vec<Rc<RefCell<VMStackItem>>>,
	reference_counter: Rc<RefCell<ReferenceCounter>>,
}

impl Slot {
	pub fn new(
		items: Vec<Rc<RefCell<VMStackItem>>>,
		reference_counter: Rc<RefCell<ReferenceCounter>>,
	) -> Self {
		let mut slot = Self { items, reference_counter };
		for item in &slot.items {
			slot.reference_counter.add_stack_reference(item, 1);
		}
		slot
	}

	pub fn new_with_count(count: i32, reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		let mut items = Vec::new();
		for _ in 0..count {
			items.push(StackItem::from(Null::default()).into());
		}

		Self { items, reference_counter }
	}

	pub fn with_capacity(
		capacity: usize,
		reference_counter: Rc<RefCell<ReferenceCounter>>,
	) -> Self {
		Self { items: Vec::with_capacity(capacity), reference_counter }
	}

	pub fn get(&self, index: usize) -> Rc<RefCell<VMStackItem>> {
		self.items[index].clone()
	}

	pub fn set(&mut self, index: usize, value: Rc<RefCell<VMStackItem>>) {
		let old_value = std::mem::replace(&mut self.items[index], value);
		self.reference_counter.remove_stack_reference(&old_value);
		self.reference_counter.add_stack_reference(&value, 1);
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn clear_references(&mut self) {
		for item in &self.items {
			self.reference_counter.get_mut().remove_stack_reference(item.clone());
		}
	}
}

impl IntoIterator for Slot {
	type Item = Rc<RefCell<VMStackItem>>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}
