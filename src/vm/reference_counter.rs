#![feature(linked_list_remove)]

use crate::types::stack_item::{ObjectReferenceEntry, StackItem, StackItemWrapper};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};
pub struct ReferenceCounter {
	tracked_items: HashMap<Rc<RefCell<StackItem>>, ()>,
	zero_referred: HashMap<Rc<RefCell<StackItem>>, ()>,
	references_count: usize,
}

impl ReferenceCounter {
	pub fn new() -> Self {
		ReferenceCounter {
			tracked_items: HashMap::new(),
			zero_referred: HashMap::new(),
			references_count: 0,
		}
	}

	pub fn add_reference(&mut self, item: Rc<RefCell<StackItemWrapper>>, parent: Rc<RefCell<StackItemWrapper>>) {
		self.references_count += 1;
		if !Self::need_track(&item.borrow()) {
			return;
		}
		self.tracked_items.insert(Rc::clone(&item), ());
		let mut item = item.borrow_mut();
		item.object_references
			.get_or_insert_with(HashMap::new)
			.entry(Rc::clone(&parent))
			.or_insert_with(|| ObjectReferenceEntry {
				item: Rc::clone(&parent),
				references: 0,
			})
			.references += 1;
	}

	pub fn add_stack_reference(&mut self, item: Rc<RefCell<StackItem>>, count: usize) {
		self.references_count += count;
		if !Self::need_track(&item.borrow()) {
			return;
		}
		self.tracked_items.insert(Rc::clone(&item), ());
		let mut item = item.borrow_mut();
		item.stack_references += count;
		self.zero_referred.remove(&item);
	}

	pub fn remove_reference(
		&mut self,
		item: Rc<RefCell<StackItem>>,
		parent: Rc<RefCell<StackItem>>,
	) {
		self.references_count -= 1;
		if !Self::need_track(&item.borrow()) {
			return;
		}
		let mut item = item.borrow_mut();
		if let Some(refs) = &mut item.object_references {
			if let Some(entry) = refs.get_mut(&parent) {
				entry.references -= 1;
			}
		}
		if item.stack_references == 0 {
			self.zero_referred.insert(Rc::clone(&item), ());
		}
	}

	pub fn remove_stack_reference(&mut self, item: Rc<RefCell<StackItem>>) {
		self.references_count -= 1;
		if !Self::need_track(&item.borrow()) {
			return;
		}
		let mut item = item.borrow_mut();
		item.stack_references -= 1;
		if item.stack_references == 0 {
			self.zero_referred.insert(Rc::clone(&item), ());
		}
	}

	fn need_track(item: &StackItem) -> bool {
		matches!(
			item,
			StackItem::Array(_) | StackItem::Struct(_) | StackItem::Map(_) | StackItem::Buffer(_)
		)
	}
}
