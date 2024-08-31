use crate::stack_item::StackItem;
use std::{
	collections::{HashSet, VecDeque},
	hash::{Hash, Hasher},
	iter::FromIterator,
};

pub struct Tarjan<'a> {
	stack_items: Vec<&'a VMStackItem>,
	stack: VecDeque<&'a mut VMStackItem>,
	components: Vec<HashSet<&'a mut VMStackItem>>,
	index: usize,
}

impl Tarjan {
	pub fn new(stack_items: Vec<&VMStackItem>) -> Self {
		Self { stack_items, stack: VecDeque::new(), components: Vec::new(), index: 0 }
	}

	pub fn invoke(&mut self) -> Vec<HashSet<&mut VMStackItem>> {
		for item in self.stack_items {
			if item.dfn() < 0 {
				self.strong_connect(item);
			}
		}

		self.components.clone()
	}

	fn strong_connect(&mut self, item: &VMStackItem) {
		let mut stack_item = StackItem::new(item.clone(), self.index);
		self.stack.push_back(stack_item);

		for successor in &item.into().successors {
			if successor.dfn < 0 {
				self.strong_connect(successor);
				stack_item.lowlink = stack_item.lowlink.min(successor.dfn as usize);
			} else if self.stack.contains(successor) {
				stack_item.lowlink = stack_item.lowlink.min(successor.dfn as usize);
			}
		}

		if stack_item.lowlink == stack_item.index {
			let mut component = HashSet::with_capacity(1);
			while let Some(w) = self.stack.pop_back() {
				w.set_on_stack(false);
				component.insert(w);
				if w == stack_item {
					break
				}
			}
			self.components.push(component);
		}

		self.index += 1;
	}
}
