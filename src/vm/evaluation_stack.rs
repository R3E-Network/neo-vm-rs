use std::{cell::RefCell, rc::Rc};
use std::cell::Ref;
use super::{reference_counter::ReferenceCounter, vm_error::VMError};
use crate::types::stack_item::StackItem;

pub struct EvaluationStack {
	inner_list: Vec<Rc<RefCell<StackItem>>>,
	reference_counter: Rc<RefCell<ReferenceCounter>>,
}

impl EvaluationStack {
	pub fn new(reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		EvaluationStack { inner_list: Vec::new(), reference_counter }
	}

	pub fn count(&self) -> usize {
		self.inner_list.len()
	}

	pub fn clear(&mut self) {
		for item in self.inner_list.drain(..) {
			self.reference_counter.borrow_mut().remove_stack_reference(item);
		}
	}

	pub fn copy_to(&self, stack: &mut EvaluationStack, count: Option<usize>) {
		let count = count.unwrap_or(self.count());
		if count == 0 {
			return;
		}
		let start = self.count().saturating_sub(count);
		for item in &self.inner_list[start..] {
			let cloned_item = item.clone();
			stack.inner_list.push(cloned_item.clone());
			stack.reference_counter.borrow_mut().add_stack_reference(item.clone(), 1);
		}
	}

	pub fn insert(&mut self, index: usize, item: Rc<RefCell<StackItem>>) -> Result<(), VMError> {
		if index > self.count() {
			return Err(VMError::InvalidParameter("Insert out of bounds".to_string()));
		}
		self.inner_list.insert(self.count() - index, item.clone());
		self.reference_counter.borrow_mut().add_stack_reference(item, 1);
		Ok(())
	}

	pub fn move_to(&mut self, stack: &mut EvaluationStack, count: Option<usize>) {
		let count = count.unwrap_or(self.count());
		if count == 0 {
			return;
		}
		let start = self.count().saturating_sub(count);
		for item in self.inner_list.drain(start..) {
			stack.inner_list.push(item.clone());
			stack.reference_counter.borrow_mut().add_stack_reference(item.clone(), 1);
			self.reference_counter.borrow_mut().remove_stack_reference(item);
		}
	}

	pub fn peek(&self, index: usize) -> Result<Rc<RefCell<StackItem>>, VMError> {
		if index >= self.count() {
			return Err(VMError::InvalidParameter("Peek out of bounds".to_string()));
		}
		Ok(Rc::clone(&self.inner_list[self.count() - index - 1]))
	}

	pub fn push(&mut self, item: Rc<RefCell<StackItem>>) {
		self.inner_list.push(Rc::clone(&item));
		self.reference_counter.borrow_mut().add_stack_reference(item, 1);
	}

	pub fn reverse(&mut self, n: usize) -> Result<(), VMError> {
		if n > self.count() {
			return Err(VMError::InvalidParameter("Reverse out of bounds".to_string()));
		}
		if n <= 1 {
			return Ok(());
		}
		let start = self.count() - n;
		self.inner_list[start..].reverse();
		Ok(())
	}

	pub fn pop(&mut self) -> Result<Rc<RefCell<StackItem>>, VMError> {
		self.remove(0)
	}

	fn remove(&mut self, index: usize) -> Result<Rc<RefCell<StackItem>>, VMError> {
		if index >= self.count() {
			return Err(VMError::InvalidParameter("Remove out of bounds".to_string()));
		}
		let adjusted_index = self.count() - index - 1;
		let item = self.inner_list.remove(adjusted_index);
		self.reference_counter.borrow_mut().remove_stack_reference(item.clone());
		Ok(item)
	}
}

impl std::fmt::Display for EvaluationStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[")?;
		for (i, item) in self.inner_list.iter().rev().enumerate() {
			if i > 0 {
				write!(f, ", ")?;
			}
			write!(f, "{:?}({:?})", item.borrow().get_type(), item.borrow())?;
		}
		write!(f, "]")
	}
}
