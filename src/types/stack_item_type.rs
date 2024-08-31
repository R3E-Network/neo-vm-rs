#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StackItemType {
	Any = 0x00,
	Pointer = 0x10,
	Boolean = 0x20,
	Integer = 0x21,
	ByteString = 0x28,
	Buffer = 0x30,
	Array = 0x40,
	Struct = 0x41,
	Map = 0x48,
	InteropInterface = 0x60,
}

impl StackItemType {
	pub fn is_valid(tp: u8) -> bool {
		match tp {
			0x00 | 0x10 | 0x20 | 0x21 | 0x28 | 0x30 | 0x40 | 0x41 | 0x48 | 0x60 => true,
			_ => false,
		}
	}

	pub fn is_valid_type(tp: StackItemType) -> bool {
		matches!(
			tp,
			StackItemType::Any
				| StackItemType::Pointer
				| StackItemType::Boolean
				| StackItemType::Integer
				| StackItemType::ByteString
				| StackItemType::Buffer
				| StackItemType::Array
				| StackItemType::Struct
				| StackItemType::Map
				| StackItemType::InteropInterface
		)
	}

	pub fn is_primitive(tp: u8) -> bool {
		match tp {
			0x20 | 0x21 | 0x28 => true,
			_ => false,
		}
	}

	pub fn is_primitive_type(tp: StackItemType) -> bool {
		matches!(tp, StackItemType::Boolean | StackItemType::Integer | StackItemType::ByteString)
	}

	pub fn is_compound(tp: u8) -> bool {
		match tp {
			0x40 | 0x41 | 0x48 => true,
			_ => false,
		}
	}

	pub fn is_compound_type(tp: StackItemType) -> bool {
		matches!(tp, StackItemType::Array | StackItemType::Struct | StackItemType::Map)
	}

	pub fn from_u8(value: u8) -> Option<Self> {
		match value {
			0x00 => Some(Self::Any),
			0x10 => Some(Self::Pointer),
			0x20 => Some(Self::Boolean),
			0x21 => Some(Self::Integer),
			0x28 => Some(Self::ByteString),
			0x30 => Some(Self::Buffer),
			0x40 => Some(Self::Array),
			0x41 => Some(Self::Struct),
			0x48 => Some(Self::Map),
			0x60 => Some(Self::InteropInterface),
			_ => None,
		}
	}
}

impl From<u8> for StackItemType {
	fn from(value: u8) -> Self {
		Self::from_u8(value).unwrap()
	}
}
