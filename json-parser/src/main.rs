use std::{fs, };
use json::{self, object};

fn main() {
	let file_path = "joke.json";
	let file_contents = fs::read_to_string(file_path).unwrap();
	let parsed = json::parse(&file_contents).unwrap();

}
