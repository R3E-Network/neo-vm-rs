use crate::types::compound_types::vm_array::VMArray;
use crate::types::compound_types::vm_map::VMMap;
use crate::types::compound_types::vm_struct::VMStruct;
use crate::types::vm_stack_item::VMStackItem;

pub enum VMCompound{
    Map(VMMap),
    Array(VMArray),
    Struct(VMStruct),
}


impl Into<VMStackItem> for VMCompound{
    fn into(self) -> VMStackItem {
        match self {
            VMCompound::Map(vm_map) => VMStackItem::Map(vm_map),
            VMCompound::Array(vm_array) => VMStackItem::Array(vm_array),
            VMCompound::Struct(vm_struct) => VMStackItem::Struct(vm_struct),
        }
    }
}

impl From<VMMap> for VMCompound{
	fn from(vm_map: VMMap) -> Self {
		VMCompound::Map(vm_map)
	}
}

impl From<VMArray> for VMCompound {
	fn from(vm_array: VMArray) -> Self {
		VMCompound::Array(vm_array)
	}
}

impl From<VMStruct> for VMCompound {
	fn from(vm_struct: VMStruct) -> Self {
		VMCompound::Struct(vm_struct)
	}
}