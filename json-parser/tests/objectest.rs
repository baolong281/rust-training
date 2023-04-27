#[cfg(test)]
mod tests {
	use jsonparser::jsonObject::JsonObject;

	#[test]
	fn test_one() {
		let mut obj = JsonObject::new();
		obj.put("fortnite", 1);
		println!("{obj.get("fortnite"):?}");
	}

}