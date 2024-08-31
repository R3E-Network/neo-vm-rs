use std::{
	cell::RefCell,
	ops::{Index, IndexMut},
	rc::Rc,
};

use crate::types::stack_item::{StackItem, StackItemWrapper};

use super::reference_counter::ReferenceCounter;

/// Used to store local variables, arguments and static fields in the VM.
pub struct Slot {
	reference_counter: Rc<RefCell<ReferenceCounter>>,
	items: Vec<Rc<RefCell<StackItem>>>,
}

impl Slot {
	/// Creates a slot containing the specified items.
	pub fn new(
		items: Vec<Rc<RefCell<StackItem>>>,
		reference_counter: Rc<RefCell<ReferenceCounter>>,
	) -> Self {
		let slot = Slot { reference_counter: Rc::clone(&reference_counter), items };

		// Add stack references for all items
		for item in &slot.items {
			reference_counter.borrow_mut().add_stack_reference(Rc::clone(item));
		}

		slot
	}

	/// Create a slot of the specified size.
	pub fn with_count(count: usize, reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		let items = vec![Rc::new(RefCell::new(StackItem::Null)); count];
		let slot = Slot { reference_counter: Rc::clone(&reference_counter), items };

		// Add stack references for all null items
		reference_counter
			.borrow_mut()
			.add_stack_reference(Rc::new(RefCell::new(StackItem::Null)), count);

		slot
	}

	/// Gets the number of items in the slot.
	pub fn count(&self) -> usize {
		self.items.len()
	}

	/// Clears all references in the slot.
	pub fn clear_references(&mut self) {
		for item in &self.items {
			self.reference_counter.borrow_mut().remove_stack_reference(item.clone());
		}
	}
}

impl Index<usize> for Slot {
	type Output = Rc<RefCell<StackItemWrapper>>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.items[index]
	}
}

impl IndexMut<usize> for Slot {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.items[index]
	}
}

impl IntoIterator for Slot {
	type Item = Rc<RefCell<StackItemWrapper>>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'a> IntoIterator for &'a Slot {
	type Item = &'a Rc<RefCell<StackItemWrapper>>;
	type IntoIter = std::slice::Iter<'a, Rc<RefCell<StackItemWrapper>>>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter()
	}
}
