use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::types::vm_stack_item::VMStackItem;
use crate::vm::reference_counter::ReferenceCounter;
use crate::types::stack_item::StackItem;

pub struct EvaluationStack {
	inner_list: VecDeque<Rc<RefCell<dyn StackItem>>>,
	reference_counter: Rc<RefCell<ReferenceCounter>>,
}

impl EvaluationStack {

	pub fn new(reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		Self {
			inner_list: VecDeque::new(),
			reference_counter,
		}
	}

    pub fn clear(&mut self) {
        for item in self.inner_list.iter() {
            self.reference_counter.borrow_mut().remove_stack_reference(item);
        }
        self.inner_list.clear();
    }

    pub fn copy_to(&self, stack: &mut EvaluationStack, count: Option<usize>) {
        let count = count.unwrap_or(self.inner_list.len());
        if count == 0 {
            return;
        }
        let start = self.inner_list.len().saturating_sub(count);
        stack.inner_list.extend_from_slice(&self.inner_list[start..]);
        for item in &self.inner_list[start..] {
            stack.reference_counter.borrow_mut().add_stack_reference(item);
        }
    }

	pub fn insert(&mut self, index: usize, item: Rc<RefCell<VMStackItem>>) {
		if index > self.inner_list.len() {
			panic!("Insert out of bounds");
		}
		self.inner_list.insert(self.inner_list.len() - index, item);
		self.reference_counter.add_stack_reference(&item);
	}

	pub fn move_to(&mut self, stack: &mut EvaluationStack, count: i32) {
		if count == 0 {
			return;
		}
		self.copy_to(stack, count);
		if count == -1 || count as usize == self.inner_list.len() {
			self.inner_list.clear();
		} else {
			let end = self.inner_list.len() - count as usize;
			self.inner_list.drain(end..);
		}
	}

	pub fn peek(&self, index: i32) -> Rc<RefCell<VMStackItem>> {
		let index = index as isize;
		if index >= self.inner_list.len() as isize {
			panic!("Peek out of bounds");
		}
		if index < 0 {
			let index = self.inner_list.len() as isize + index;
			if index < 0 {
				panic!("Peek out of bounds");
			}
		}
		self.inner_list.get((self.inner_list.len() as isize - index - 1) as usize).unwrap().clone()
	}

	pub fn push(&mut self, item: Rc<RefCell<VMStackItem>>) {
		self.inner_list.push_back(item);
		self.reference_counter.add_stack_reference(&item);
	}

	pub fn reverse(&mut self, n: i32) {
		let n = n as usize;
		if n < 0 || n > self.inner_list.len() {
			panic!("Argument out of range");
		}
		if n <= 1 {
			return;
		}
		let end = self.inner_list.len() - n;
		self.inner_list.make_contiguous().reverse();
	}

	pub fn pop(&mut self) -> Rc<RefCell<VMStackItem>> {
		self.remove(0)
	}

	pub fn pop_typed<T: StackItem>(&mut self) -> T {
		self.remove::<T>(0)
	}

	pub fn remove<T: StackItem>(&mut self, index: i32) -> T {
		let index = index as isize;
		if index >= self.inner_list.len() as isize {
			panic!("Argument out of range");
		}
		if index < 0 {
			let index = self.inner_list.len() as isize + index;
			if index < 0 {
				panic!("Argument out of range");
			}
		}
		let index = self.inner_list.len() as isize - index - 1;
		let item = self.inner_list.remove(index as usize).unwrap();
		if !item.is::<T>() {
			panic!("Invalid cast");
		}
		self.reference_counter.remove_stack_reference(&item);
		item.try_into().unwrap()
	}

	pub fn size(&self) -> usize {
		self.inner_list.len()
	}
}