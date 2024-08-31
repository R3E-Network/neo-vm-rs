#![feature(associated_type_defaults)]
#![feature(linked_list_remove)]
#![feature(exclusive_range_pattern)]

extern crate core;

pub use num_bigint::BigInt;
pub mod exception;
pub mod script;
// pub mod types;

mod jump_table;
mod types;
pub mod vm;

pub use exception::*;
pub use script::*;
// pub use types::*;
pub use vm::*;

pub mod utility {
	use super::*;

	pub fn strict_utf8_decode(bytes: &[u8]) -> Option<String> {
		std::str::from_utf8(bytes).ok().map(|s| s.to_string())
	}

	pub fn mod_inverse(value: &BigInt, modulus: &BigInt) -> Option<BigInt> {
		if value <= &BigInt::from(0) || modulus < &BigInt::from(2) {
			return None;
		}
		let (mut r, mut old_r) = (value.clone(), modulus.clone());
		let (mut s, mut old_s) = (BigInt::from(1), BigInt::from(0));
		while r > BigInt::from(0) {
			let q = &old_r / &r;
			let temp_r = r.clone();
			r = old_r - &q * &temp_r;
			old_r = temp_r;
			let temp_s = s.clone();
			s = old_s - &q * &temp_s;
			old_s = temp_s;
		}
		let mut result = old_s % modulus;
		if result < BigInt::from(0) {
			result += modulus;
		}
		if (&value * &result % modulus) != BigInt::from(1) {
			return None;
		}
		Some(result)
	}

	pub fn sqrt(value: &BigInt) -> Option<BigInt> {
		if value < &BigInt::from(0) {
			return None;
		}
		if value == &BigInt::from(0) {
			return Some(BigInt::from(0));
		}
		if value < &BigInt::from(4) {
			return Some(BigInt::from(1));
		}
		let mut z = value.clone();
		let mut x = (BigInt::from(1) << (((value - 1) as BigInt).bits() as u32 + 1) >> 1);
		while &x < &z {
			z = x.clone();
			x = (value / &x + &x) / 2;
		}
		Some(z)
	}

	pub fn get_bit_length(value: &BigInt) -> u32 {
		if value == &BigInt::from(0) || value == &BigInt::from(-1) {
			return 0;
		}
		value.bits() as u32
	}
}

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
