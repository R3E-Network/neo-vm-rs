use std::collections::HashMap;
use serde::__private::de::Content::String;
use crate::types::primitive_types::vm_byte_string::VMByteString;
use crate::types::primitive_types::vm_integer::VMInteger;
use crate::types::stack_item::StackItem;
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_buffer::VMBuffer;
use crate::types::vm_stack_item::VMStackItem;
use crate::vm_error::VMError;

pub trait PrimitiveType: StackItem {
	fn memory(&self) -> Vec<u8>;

	/// The size of the vm object in bytes.
	fn size(&self) -> usize {
		self.memory().len()
	}

	fn convert_to(&self, type_: StackItemType) -> Result<VMStackItem, VMError>  {
		match type_ {
			StackItemType::Integer => Ok(VMInteger::from(self.get_integer())),
			StackItemType::ByteString =>  Ok(VMByteString::from( String::from_utf8(self.memory())?)),
			StackItemType::Buffer =>  Ok(VMBuffer::from(self.get_slice()).into()),
			StackItemType::Boolean =>  Ok(VMBoolean::from(self.get_boolean().into()).into()),
			_ => panic!(), //self.base_convert_to(ty),
		}
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>) -> Box<VMStackItem> {
		
	}

	fn get_slice(&self) -> Vec<u8>{
		self.memory()
	}
}
