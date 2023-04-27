use std::collections::HashMap;
use json::JsonValue;
use std::ops::{Index, };
static NULL: JsonValue = JsonValue::Null;

pub struct JsonObject {
	items: HashMap<&'static str, JsonValue>,
}

impl JsonObject {
	pub fn new() -> JsonObject {
		let items: HashMap<&'static str, JsonValue> = HashMap::new();
		JsonObject { items }
	}

	pub fn put<T>(&mut self, key: &'static str, val: T) 
	where json::JsonValue: From<T>,
	{
		self.items.insert(key, JsonValue::from(val));
	}

	pub fn get(&self, key: &'static str) -> &JsonValue {
		match self.items.get(key) {
			Some(val) => val,
			_ => &NULL
		}
	}
}

impl Index<&'static str> for JsonObject {
	type Output = JsonValue;

	fn index(&self, index: &'static str) -> &Self::Output {
		match self.items.get(index) {
			Some(value) => value,
			_ => &NULL
		}
	}
}