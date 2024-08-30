use std::{
	cell::RefCell,
	fmt::{Debug},
	hash::{Hash, Hasher},
	rc::Rc,
	string::FromUtf8Error,
};
use std::any::Any;
use std::collections::HashMap;
use num_bigint::BigInt;
use crate::{vm::execution_engine_limits::ExecutionEngineLimits, vm_error::VMError};
use crate::types::vm_interop_interface::VMInteropInterface;
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_null::VMNull;
use crate::types::vm_stack_item::VMStackItem;

use super::compound_types::vm_compound::VMCompound;

pub trait StackItem  : Any {

	fn dfn(&self) -> isize{
		todo!()
	}

	fn set_dfn(&mut self, dfn: isize){
		todo!()
	}

	fn low_link(&self) -> usize{
		todo!()
	}
	fn set_low_link(&mut self, link: usize){
		todo!()
	}

	fn on_stack(&self) -> bool{
		todo!()
	}
	fn set_on_stack(&mut self, on_stack: bool){
		todo!()
	}

	fn set_object_references(&mut self, refs: RefCell<HashMap<VMCompound, ObjectReferenceEntry>>) {
		todo!()
	}

	fn object_references(&self) -> RefCell<HashMap<VMCompound, ObjectReferenceEntry>> {
		todo!()
	}

	fn set_stack_references(&mut self, count: usize){
		todo!()
	}

	fn stack_references(&self) -> usize{
		todo!()
	}

	fn successors(&self) -> Vec<VMStackItem> {
		self.object_references()
			.borrow()
			.as_ref()
			.unwrap()
			.values()
			.map(|v| v.item())
			.collect()
	}

	fn reset(&mut self) {
		self.set_dfn(-1);
		self.set_low_link(0);
		self.set_on_stack(false);
	}

	fn is_null(&self) -> bool {
		match self.get_type(){
			StackItemType::Any => true,
			_ => false,
		}
	}

	fn cleanup(&mut self);

	fn convert_to(&self, type_: StackItemType) -> Result<VMStackItem, VMError> {
		if type_ == self.get_type() {
			Ok(self.to_owned())
		} else if type_ == StackItemType::Boolean {
			Ok(self.get_boolean())
		} else {
			Err(VMError::new("Cannot convert to the given type"))
		}
	}

	fn get_slice(&self) -> Vec<u8>;

	fn get_string(&self) -> Result<String, FromUtf8Error> {
		String::from_utf8(self.get_slice().to_vec())
	}

	fn get_hash_code(&self) -> u64 {
		// use std::hash::Hasher;
		// let mut hasher = std::collections::hash_map::DefaultHasher::new();
		// self.hash(&mut hasher);
		// hasher.finish()
		todo!()
	}

	fn get_type(&self) -> StackItemType;

	fn get_boolean(&self) -> bool;


	fn deep_copy(&self, asImmutable:bool) -> Box<VMStackItem>;

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable:bool) -> Box<VMStackItem>;

	fn equals(&self, other: &VMStackItem) -> bool;

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool;

	fn from_interface(value: Option<Box<dyn Any>>) -> Box<VMStackItem>{

		match value {
			Some(value)=>VMInteropInterface::new(value),	
			None => VMNull::new(),
		}
	}
	fn get_integer(&self) -> BigInt;

	fn get_interface<T: Any>(&self) -> Option<&T>{
		panic!("Not implemented")
	}


	fn get_bytes(&self) -> &[u8];

	fn to_ref(&self) -> Rc<RefCell<VMStackItem>> {
		Rc::new(RefCell::new(self.clone()))
	}

}

pub struct ObjectReferenceEntry {
	pub(crate) item: Rc<RefCell<VMStackItem>>,
	pub(crate) references: i32,
}

impl ObjectReferenceEntry {
	pub fn new(item: Rc<RefCell<VMStackItem>>) -> Self {
		Self { item, references: 0 }
	}
}
