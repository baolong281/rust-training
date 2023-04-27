mod tests {
	use jsonparser::JsonObject;

	#[test]
	fn test_one() {
		let mut obj = JsonObject::new();
		obj.put("a", "cheese");
		println!("{:?}", obj.get("a"));
	}
}