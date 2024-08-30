use std::cell::{Ref, RefCell};

use crate::types::stack_item::StackItem;
use crate::types::vm_stack_item::VMStackItem;

pub trait CompoundType: StackItem {
	fn count(&self) -> usize;
	fn sub_items(&self) -> Vec<Ref<RefCell<VMStackItem>>>;
	fn sub_items_count(&self) -> usize{
		self.sub_items().len()
	}
	fn read_only(&mut self);
	fn is_read_only(&self) -> bool {
		false
	}

	fn clear(&mut self);

	fn as_bool(&self) -> bool {
		true
	}
}