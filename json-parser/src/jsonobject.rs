use std::collections::HashMap;
use json::JsonValue;
use std::ops::{Index, IndexMut};
static NULL: JsonValue = JsonValue::Null;

pub mod jsonObject {

	use std::process::Output;

use super::*;

	struct JsonObject {
		items: HashMap<&'static str, JsonValue>,
	}

	impl JsonObject {
		pub fn new() -> JsonObject {
			let items: HashMap<&'static str, JsonValue> = HashMap::new();
			JsonObject { items }
		}

		pub fn put(&mut self, key: &'static str, val: JsonValue) {
			self.items.insert(key, val);
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
}