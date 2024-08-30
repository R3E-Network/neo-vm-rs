pub mod vm_interop_interface;
pub mod stack_item;
pub mod stack_item_type;

pub mod vm_buffer;
pub mod vm_null;
pub mod vm_pointer;
pub mod compound_types;
pub mod primitive_types;
pub mod vm_stack_item;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
