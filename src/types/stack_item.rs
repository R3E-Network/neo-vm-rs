use std::{
	any::Any,
	cell::RefCell,
	collections::HashMap,
	hash::{Hash, Hasher},
	rc::Rc,
};

use num_bigint::BigInt;

use crate::execution_engine_limits::ExecutionEngineLimits;

use super::stack_item_type::StackItemType;

#[derive(Clone, Debug)]
enum StackItem {
	Boolean(bool),
	Integer(BigInt),
	ByteString(Vec<u8>),
	Buffer(Vec<u8>),
	Array(Vec<Rc<RefCell<StackItem>>>),
	Struct(Vec<Rc<RefCell<StackItem>>>),
	Map(HashMap<StackItem, Rc<RefCell<StackItem>>>),
	InteropInterface(Rc<dyn Any>),
	Null,
}

// Rename StackItemData to StackItemMetadata for clarity
#[derive(Clone, Debug)]
struct StackItemMetadata {
	pub stack_references: usize,
	pub object_references: Option<HashMap<Rc<RefCell<StackItem>>, ObjectReferenceEntry>>,
	pub dfn: i32,
	pub low_link: i32,
	pub on_stack: bool,
}

// New wrapper struct
#[derive(Clone, Debug)]
pub struct StackItemWrapper {
	pub item: StackItem,
	pub metadata: StackItemMetadata,
}

#[derive(Clone, Debug)]
pub struct ObjectReferenceEntry {
	pub(crate) item: Rc<RefCell<StackItem>>,
	pub(crate) references: usize,
}

impl StackItem {
	pub fn new() -> (Self, Rc<RefCell<StackItemMetadata>>) {
		(
			StackItem::Null,
			Rc::new(RefCell::new(StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			})),
		)
	}

	pub fn get_type(&self) -> StackItemType {
		match self {
			StackItem::Boolean(_) => StackItemType::Boolean,
			StackItem::Integer(_) => StackItemType::Integer,
			StackItem::ByteString(_) => StackItemType::ByteString,
			StackItem::Buffer(_) => StackItemType::Buffer,
			StackItem::Array(_) => StackItemType::Array,
			StackItem::Struct(_) => StackItemType::Struct,
			StackItem::Map(_) => StackItemType::Map,
			StackItem::InteropInterface(_) => StackItemType::InteropInterface,
			StackItem::Null => StackItemType::Any,
		}
	}

	pub fn successors(&self, data: &StackItemMetadata) -> Vec<Rc<RefCell<StackItem>>> {
		match &data.object_references {
			Some(refs) => refs
				.values()
				.filter(|entry| entry.references > 0)
				.map(|entry| Rc::clone(&entry.item))
				.collect(),
			None => Vec::new(),
		}
	}

	pub fn reset(data: &mut StackItemMetadata) {
		data.dfn = -1;
		data.low_link = 0;
		data.on_stack = false;
	}

	pub fn is_null(&self) -> bool {
		matches!(self, StackItem::Null)
	}

	pub fn get_span(&self) -> Vec<u8> {
		match self {
			StackItem::Boolean(b) => vec![*b as u8],
			StackItem::Integer(i) => i.to_signed_bytes_le(),
			StackItem::ByteString(bs) => bs.clone(),
			StackItem::Buffer(b) => b.clone(),
			_ => Vec::new(),
		}
	}

	pub fn get_boolean(&self) -> bool {
		match self {
			StackItem::Boolean(b) => *b,
			StackItem::Integer(i) => i != &BigInt::from(0),
			StackItem::ByteString(bs) => !bs.is_empty(),
			StackItem::Buffer(b) => !b.is_empty(),
			StackItem::Null => false,
			_ => true,
		}
	}

	pub fn get_integer(&self) -> Result<BigInt, &'static str> {
		match self {
			StackItem::Boolean(b) => Ok(BigInt::from(*b as u8)),
			StackItem::Integer(i) => Ok(i.clone()),
			StackItem::ByteString(bs) | StackItem::Buffer(bs) =>
				if bs.len() > 8 {
					Err("ByteString or Buffer too long for integer conversion")
				} else {
					let mut bytes = [0u8; 8];
					bytes[..bs.len()].copy_from_slice(bs);
					Ok(BigInt::from_signed_bytes_le(bs))
				},
			_ => Err("Cannot convert to integer"),
		}
	}

	pub fn convert_to(&self, item_type: StackItemType) -> Result<StackItem, &'static str> {
		if self.get_type() == item_type {
			return Ok(self.clone());
		}
		match item_type {
			StackItemType::Boolean => Ok(StackItem::Boolean(self.get_boolean())),
			StackItemType::Integer => self.get_integer().map(StackItem::Integer),
			StackItemType::ByteString => Ok(StackItem::ByteString(self.get_span().to_vec())),
			StackItemType::Buffer => Ok(StackItem::Buffer(self.get_span().to_vec())),
			_ => Err("Invalid conversion"),
		}
	}

	pub fn deep_copy(
		&self,
		ref_map: &mut HashMap<RcRefCellWrapper, Rc<RefCell<StackItem>>>,
		as_immutable: bool,
	) -> Rc<RefCell<StackItem>> {
		match self {
			StackItem::Array(items) | StackItem::Struct(items) => {
				let new_items = items
					.iter()
					.map(|item| {
						let item_ref = RcRefCellWrapper(Rc::clone(item));
						ref_map.get(&item_ref).cloned().unwrap_or_else(|| {
							let new_item = item.borrow().deep_copy(ref_map, as_immutable);
							ref_map.insert(item_ref, Rc::clone(&new_item));
							new_item
						})
					})
					.collect();

				Rc::new(RefCell::new(if as_immutable {
					StackItem::Array(new_items)
				} else {
					match self {
						StackItem::Array(_) => StackItem::Array(new_items),
						StackItem::Struct(_) => StackItem::Struct(new_items),
						_ => unreachable!(),
					}
				}))
			},
			StackItem::Map(map) => {
				let new_map = map
					.iter()
					.map(|(k, v)| {
						let v_ref = RcRefCellWrapper(Rc::clone(v));
						let new_v = ref_map.get(&v_ref).cloned().unwrap_or_else(|| {
							let new_item = v.borrow().deep_copy(ref_map, as_immutable);
							ref_map.insert(v_ref, Rc::clone(&new_item));
							new_item
						});
						(k.clone(), new_v)
					})
					.collect();
				Rc::new(RefCell::new(StackItem::Map(new_map)))
			},
			_ => Rc::new(RefCell::new(self.clone())),
		}
	}

	pub fn equals(&self, other: &StackItem, limits: &ExecutionEngineLimits) -> bool {
		if std::ptr::eq(self, other) {
			return true;
		}
		match (self, other) {
			(StackItem::Boolean(a), StackItem::Boolean(b)) => a == b,
			(StackItem::Integer(a), StackItem::Integer(b)) => a == b,
			(StackItem::ByteString(a), StackItem::ByteString(b))
			| (StackItem::Buffer(a), StackItem::Buffer(b)) => a == b,
			(StackItem::Array(a), StackItem::Array(b))
			| (StackItem::Struct(a), StackItem::Struct(b)) => {
				if a.len() != b.len() {
					return false;
				}
				for (item_a, item_b) in a.iter().zip(b.iter()) {
					if !item_a.borrow().equals(&item_b.borrow(), limits) {
						return false;
					}
				}
				true
			},
			(StackItem::Map(a), StackItem::Map(b)) => {
				if a.len() != b.len() {
					return false;
				}
				for (key, value_a) in a {
					if let Some(value_b) = b.get(key) {
						if !value_a.borrow().equals(&value_b.borrow(), limits) {
							return false;
						}
					} else {
						return false;
					}
				}
				true
			},
			_ => false,
		}
	}

	pub fn get_interface<T: 'static>(&self) -> Option<&T> {
		if let StackItem::InteropInterface(boxed) = self {
			boxed.downcast_ref()
		} else {
			None
		}
	}

	pub fn get_string(&self) -> Option<String> {
		match self {
			StackItem::ByteString(bs) | StackItem::Buffer(bs) => String::from_utf8(bs.clone()).ok(),
			_ => None,
		}
	}
}

impl Hash for StackItem {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.get_span().hash(state);
	}
}

impl PartialEq for StackItem {
	fn eq(&self, other: &Self) -> bool {
		self.get_span() == other.get_span()
	}
}

impl Eq for StackItem {}

struct RcRefCellWrapper(Rc<RefCell<StackItem>>);

impl Hash for RcRefCellWrapper {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.borrow().hash(state);
	}
}

impl PartialEq for RcRefCellWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0.borrow().eq(&other.0.borrow())
	}
}

impl Eq for RcRefCellWrapper {}

impl StackItemWrapper {
	pub fn new() -> Self {
		StackItemWrapper {
			item: StackItem::Null,
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn get_type(&self) -> StackItemType {
		self.item.get_type()
	}

	pub fn successors(&self) -> Vec<Rc<RefCell<StackItem>>> {
		match &self.metadata.object_references {
			Some(refs) => refs
				.values()
				.filter(|entry| entry.references > 0)
				.map(|entry| Rc::clone(&entry.item))
				.collect(),
			None => Vec::new(),
		}
	}

	pub fn reset(&mut self) {
		self.metadata.dfn = -1;
		self.metadata.low_link = 0;
		self.metadata.on_stack = false;
	}

	pub fn is_null(&self) -> bool {
		self.item.is_null()
	}

	pub fn get_span(&self) -> Vec<u8> {
		self.item.get_span()
	}

	pub fn get_boolean(&self) -> bool {
		self.item.get_boolean()
	}

	pub fn get_integer(&self) -> Result<BigInt, &'static str> {
		self.item.get_integer()
	}

	pub fn convert_to(&self, item_type: StackItemType) -> Result<StackItem, &'static str> {
		self.item.convert_to(item_type)
	}

	pub fn deep_copy(
		&self,
		ref_map: &mut HashMap<RcRefCellWrapper, Rc<RefCell<StackItem>>>,
		as_immutable: bool,
	) -> Rc<RefCell<StackItem>> {
		self.item.deep_copy(ref_map, as_immutable)
	}

	pub fn equals(&self, other: &StackItem, limits: &ExecutionEngineLimits) -> bool {
		self.item.equals(other, limits)
	}

	pub fn get_interface<T: 'static>(&self) -> Option<&T> {
		self.item.get_interface()
	}

	pub fn get_string(&self) -> Option<String> {
		self.item.get_string()
	}

	// New getter and setter methods for metadata
	pub fn get_stack_references(&self) -> usize {
		self.metadata.stack_references
	}

	pub fn set_stack_references(&mut self, value: usize) {
		self.metadata.stack_references = value;
	}

	pub fn get_object_references(&self) -> &Option<HashMap<Rc<RefCell<StackItem>>, ObjectReferenceEntry>> {
		&self.metadata.object_references
	}

	pub fn set_object_references(&mut self, value: Option<HashMap<Rc<RefCell<StackItem>>, ObjectReferenceEntry>>) {
		self.metadata.object_references = value;
	}

	pub fn get_dfn(&self) -> i32 {
		self.metadata.dfn
	}

	pub fn set_dfn(&mut self, value: i32) {
		self.metadata.dfn = value;
	}

	pub fn get_low_link(&self) -> i32 {
		self.metadata.low_link
	}

	pub fn set_low_link(&mut self, value: i32) {
		self.metadata.low_link = value;
	}

	pub fn is_on_stack(&self) -> bool {
		self.metadata.on_stack
	}

	pub fn set_on_stack(&mut self, value: bool) {
		self.metadata.on_stack = value;
	}

	// New methods to create StackItem from StackItemWrapper
	pub fn new_boolean(value: bool) -> Self {
		StackItemWrapper {
			item: StackItem::Boolean(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}
	pub fn new_integer(value: BigInt) -> Self {
		StackItemWrapper {
			item: StackItem::Integer(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_byte_string(value: Vec<u8>) -> Self {
		StackItemWrapper {
			item: StackItem::ByteString(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}
	pub fn new_buffer(value: Vec<u8>) -> Self {
		StackItemWrapper {
			item: StackItem::Buffer(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_array(value: Vec<Rc<RefCell<StackItem>>>) -> Self {
		StackItemWrapper {
			item: StackItem::Array(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_struct(value: Vec<Rc<RefCell<StackItem>>>) -> Self {
		StackItemWrapper {
			item: StackItem::Struct(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_map(value: HashMap<StackItem, Rc<RefCell<StackItem>>>) -> Self {
		StackItemWrapper {
			item: StackItem::Map(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_interop_interface(value: Rc<dyn Any>) -> Self {
		StackItemWrapper {
			item: StackItem::InteropInterface(value),
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}

	pub fn new_null() -> Self {
		StackItemWrapper {
			item: StackItem::Null,
			metadata: StackItemMetadata {
				stack_references: 0,
				object_references: None,
				dfn: -1,
				low_link: 0,
				on_stack: false,
			},
		}
	}
}
