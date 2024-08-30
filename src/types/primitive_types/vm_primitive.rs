use std::collections::HashMap;
use num_bigint::BigInt;
use crate::execution_engine_limits::ExecutionEngineLimits;
use crate::types::primitive_types::primitive_type::PrimitiveType;
use crate::types::primitive_types::vm_boolean::VMBoolean;
use crate::types::primitive_types::vm_byte_string::VMByteString;
use crate::types::primitive_types::vm_integer::VMInteger;
use crate::types::stack_item::StackItem;
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_stack_item::VMStackItem;

pub enum VMPrimitive{
    Boolean(VMBoolean),
    ByteString(VMByteString),
    Integer(VMInteger),
}

impl Into<VMStackItem> for VMPrimitive{
    fn into(self) -> VMStackItem {
        match self {
            VMPrimitive::Boolean(vm_boolean) => VMStackItem::Boolean(vm_boolean),
            VMPrimitive::ByteString(vm_byte_string) => VMStackItem::ByteString(vm_byte_string),
            VMPrimitive::Integer(vm_integer) => VMStackItem::Integer(vm_integer),
        }
    }
}

impl From<VMBoolean> for VMPrimitive{
    fn from(vm_boolean: VMBoolean) -> Self {
        VMPrimitive::Boolean(vm_boolean)
    }
}

impl From<VMByteString> for VMPrimitive{
    fn from(vm_byte_string: VMByteString) -> Self {
        VMPrimitive::ByteString(vm_byte_string)
    }
}

impl From<VMInteger> for VMPrimitive{
    fn from(vm_integer: VMInteger) -> Self {
        VMPrimitive::Integer(vm_integer)
    }
}

impl PrimitiveType for VMPrimitive {
    fn memory(&self) -> &[u8] {
        todo!()
    }
}

impl StackItem for VMPrimitive{

    fn dfn(&self) -> isize {
        todo!()
    }

    fn set_dfn(&mut self, dfn: isize) {
        todo!()
    }

    fn low_link(&self) -> usize {
        todo!()
    }

    fn set_low_link(&mut self, link: usize) {
        todo!()
    }

    fn on_stack(&self) -> bool {
        todo!()
    }

    fn set_on_stack(&mut self, on_stack: bool) {
        todo!()
    }

    fn set_object_references(&mut self, refs: Self::ObjectReferences) {
        todo!()
    }

    fn object_references(&self) -> &Self::ObjectReferences {
        todo!()
    }

    fn set_stack_references(&mut self, count: usize) {
        todo!()
    }

    fn stack_references(&self) -> usize {
        todo!()
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