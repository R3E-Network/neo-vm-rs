use std::cell::RefCell;
use std::collections::{HashMap};
use std::hash::{Hash};
use num_bigint::BigInt;
use crate::execution_engine_limits::ExecutionEngineLimits;
use crate::types::compound_types::vm_array::VMArray;
use crate::types::compound_types::vm_map::VMMap;
use crate::types::compound_types::vm_struct::VMStruct;
use crate::types::primitive_types::vm_byte_string::VMByteString;
use crate::types::primitive_types::vm_integer::VMInteger;
use crate::types::stack_item::StackItem;
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_buffer::VMBuffer;
use crate::types::vm_interop_interface::VMInteropInterface;
use crate::types::vm_null::VMNull;
use crate::types::vm_pointer::VMPointer;

use super::compound_types::vm_compound::VMCompound;
use super::primitive_types::vm_boolean::VMBoolean;
use super::stack_item::ObjectReferenceEntry;

pub enum VMStackItem {
   Boolean(VMBoolean),
   ByteString(VMByteString),
   Integer(VMInteger),
   Array(VMArray),
   Null(VMNull),
   Pointer(VMPointer),
   Struct(VMStruct),
   InteropInterface(VMInteropInterface),
   Map(VMMap),
   Buffer(VMBuffer)
}

impl VMStackItem{

   // check if its a Primitive type
    pub fn is_primitive(&self) -> bool {
        match self {
            VMStackItem::Boolean(_) => true,
            VMStackItem::ByteString(_) => true,
            VMStackItem::Integer(_) => true,
            _ => false
        }
    }

    // check if its a Compound type
    pub fn is_compound(&self) -> bool {
        match self {
            VMStackItem::Array(_) => true,
            VMStackItem::Struct(_) => true,
            VMStackItem::Map(_) => true,
            _ => false
        }
    }
}

impl StackItem for VMStackItem{

   fn dfn(&self) -> isize {
      match self{
         VMStackItem::Array(vm_array) => vm_array.dfn(),
         VMStackItem::Struct(vm_struct) => vm_struct.dfn(),
         VMStackItem::Map(vm_map) => vm_map.dfn(),
         _ => 0
      }
   }

   fn set_dfn(&mut self, dfn: isize) {
      match self{
         VMStackItem::Array(vm_array) => vm_array.set_dfn(dfn),
         VMStackItem::Struct(vm_struct) => vm_struct.set_dfn(dfn),
         VMStackItem::Map(vm_map) => vm_map.set_dfn(dfn),
         _ => ()
      }
   }

   fn low_link(&self) -> usize {
      match self{
         VMStackItem::Array(vm_array) => vm_array.low_link(),
         VMStackItem::Struct(vm_struct) => vm_struct.low_link(),
         VMStackItem::Map(vm_map) => vm_map.low_link(),
         _ => 0
      }
   }

   fn set_low_link(&mut self, link: usize) {
      match self{
         VMStackItem::Array(vm_array) => vm_array.set_low_link(link),
         VMStackItem::Struct(vm_struct) => vm_struct.set_low_link(link),
         VMStackItem::Map(vm_map) => vm_map.set_low_link(link),
         _ => ()
      }
   }

   fn on_stack(&self) -> bool {
      match self{
         VMStackItem::Array(vm_array) => vm_array.on_stack(),
         VMStackItem::Struct(vm_struct) => vm_struct.on_stack(),
         VMStackItem::Map(vm_map) => vm_map.on_stack(),
         _ => false
      }
   }

   fn set_on_stack(&mut self, on_stack: bool) {
      match self{
         VMStackItem::Array(vm_array) => vm_array.set_on_stack(on_stack),
         VMStackItem::Struct(vm_struct) => vm_struct.set_on_stack(on_stack),
         VMStackItem::Map(vm_map) => vm_map.set_on_stack(on_stack),
         _ => ()
      }
   }

	fn set_object_references(&mut self, refs: RefCell<HashMap<VMCompound, ObjectReferenceEntry>>) {
		match self{
         VMStackItem::Array(vm_array) => vm_array.set_object_references(refs),
         VMStackItem::Struct(vm_struct) => vm_struct.set_object_references(refs),
         VMStackItem::Map(vm_map) => vm_map.set_object_references(refs),
         _ => ()
      }
	}

	fn object_references(&self) -> RefCell<HashMap<VMCompound, ObjectReferenceEntry>> {
		match self{
         VMStackItem::Array(vm_array) => vm_array.object_references(),
         VMStackItem::Struct(vm_struct) => vm_struct.object_references(),
         VMStackItem::Map(vm_map) => vm_map.object_references(),
         _ => RefCell::new(HashMap::new())
      }
	}

   fn set_stack_references(&mut self, count: usize) {
      match self{
         VMStackItem::Array(vm_array) => vm_array.set_stack_references(count),
         VMStackItem::Struct(vm_struct) => vm_struct.set_stack_references(count),
         VMStackItem::Map(vm_map) => vm_map.set_stack_references(count),
         _ => ()
      }
   }

   fn stack_references(&self) -> usize {
      match self{
         VMStackItem::Array(vm_array) => vm_array.stack_references(),
         VMStackItem::Struct(vm_struct) => vm_struct.stack_references(),
         VMStackItem::Map(vm_map) => vm_map.stack_references(),
         _ => 0
      }
   }

   fn cleanup(&mut self) {
      todo!()
   }

   fn get_slice(&self) -> Vec<u8> {
      todo!()
   }

   fn get_type(&self) -> StackItemType {
      todo!()
   }

   fn get_boolean(&self) -> bool {
      todo!()
   }

   fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
      todo!()
   }

   fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
      todo!()
   }

   fn equals(&self, other: &VMStackItem) -> bool {
      todo!()
   }

   fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
      todo!()
   }

   fn get_integer(&self) -> BigInt {
      todo!()
   }

   fn get_bytes(&self) -> &[u8] {
      todo!()
   }
}