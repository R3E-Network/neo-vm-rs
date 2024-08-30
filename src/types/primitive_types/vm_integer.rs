use crate::types::{
		compound_types::{compound_type::CompoundType, vm_compound::VMCompound},
		primitive_types::primitive_type::PrimitiveType,
	};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::{
	cell::RefCell,
	collections::HashMap,
	convert::TryFrom,
	fmt::Debug,
	hash::Hash,
	ops::{Add, Div, Mul, Rem, Sub},
};
use std::any::Any;
use std::rc::Rc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::vm::execution_engine_limits::ExecutionEngineLimits;
use crate::types::stack_item::{ObjectReferenceEntry, StackItem};
use crate::types::stack_item_type::StackItemType;
use crate::types::vm_stack_item::VMStackItem;

use super::vm_primitive::VMPrimitive;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VMInteger {
	value: BigInt,
}

impl VMInteger {
	const MAX_SIZE: u32 = 32;

	pub(crate) fn new(value: &BigInt) -> Self {
		let size = value.to_bytes().len() as u32;
		if size > Self::MAX_SIZE {
			panic!("Max size exceeded: {}", size);
		}

		Self {
		
			value: value.clone(),
		}
	}
}

// Conversions

impl TryFrom<&[u8]> for VMInteger {
	type Error = ();

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		BigInt::from_bytes(value).map(|v| VMInteger::new(&v)).map_err(|_| ())
	}
}

impl From<bool> for VMInteger {
	fn from(value: bool) -> Self {
		let int_val = if value { BigInt::one() } else { BigInt::zero() };

		VMInteger::new(&int_val)
	}
}

macro_rules! from_primitive {
	($t:ty) => {
		impl From<$t> for VMInteger {
			fn from(value: $t) -> Self {
				VMInteger::new(&BigInt::from(value))
			}
		}
	};
}

from_primitive!(i8);
from_primitive!(u8);
from_primitive!(i16);
from_primitive!(u16);
from_primitive!(i32);
from_primitive!(u32);
from_primitive!(i64);
from_primitive!(u64);
from_primitive!(isize);
from_primitive!(usize);

impl Add for VMInteger {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let result = self.value + other.value;
		VMInteger::new(&result)
	}
}

impl Sub for VMInteger {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		let result = self.value - other.value;
		VMInteger::new(&result)
	}
}

impl Mul for VMInteger {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		let result = self.value * other.value;
		VMInteger::new(&result)
	}
}

impl Div for VMInteger {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		let result = self.value / other.value;
		VMInteger::new(&result)
	}
}

impl Rem for VMInteger {
	type Output = Self;

	fn rem(self, other: Self) -> Self {
		let result = self.value % other.value;
		VMInteger::new(&result)
	}
}

impl PartialEq<VMStackItem> for VMInteger {
	fn eq(&self, other: &VMStackItem) -> bool {
		self.equals(other)
	}
}

impl Serialize for VMInteger {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		serializer.serialize_bytes(self.memory())
	}
}

impl StackItem for VMInteger {
	fn cleanup(&mut self) {
		todo!()
	}

	fn get_slice(&self) -> Vec<u8> {

	if self.value.is_zero() {
		vec![]
	} else {
		let bytes = self.value.to_signed_bytes_be();
		bytes.to_vec()
	}
}

	fn get_type(&self) -> StackItemType {
		StackItemType::Integer
	}
	fn get_boolean(&self) -> bool {
		!self.value.is_zero()
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&VMStackItem, &VMStackItem>, asImmutable: bool) -> Box<VMStackItem> {
		todo!()
	}

	fn equals(&self, other: &VMStackItem) -> bool {
		if other.get_type() != StackItemType::Integer {
			return false;
		}
		self ==other || other.get_integer() == self.value
	}

	fn equals_with_limits(&self, other: &VMStackItem, limits: &ExecutionEngineLimits) -> bool {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		self.value.clone()
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl PrimitiveType for VMInteger {
	fn memory(&self) -> Vec<u8> {
		self.get_slice()
	}
}

impl From<VMPrimitive> for VMInteger {
	fn from(value: &VMPrimitive) -> Self {
		match value {
			VMPrimitive::Integer(i) => i.clone(),
			_ => panic!("Invalid cast"),
		}
	}
}
