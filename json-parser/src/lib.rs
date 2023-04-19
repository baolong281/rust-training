use std:: {
	collections::HashSet,
};

///fn lexer(json: String) -> Vec<String> {
	///let tokens: Vec<String> = Vec::new();

///}

fn lex_string(json: &mut String) -> Result<Option<String>, &'static str>{
	if json.starts_with('"') {
		json.remove(0);
	} else {
		return Ok(None);
	}

	let mut out = String::new();
	while json.len() > 0 {
		if json.starts_with('"') {
			return Ok(Some(out));
		} else {
			out.push(json.remove(0));
		}
	}

	return Err("Missing \" somewhere");
}

let mut set = HashSet::new();
for i in 0..10 {
	set.insert(i.to_string());
}
set.insert(String::from("-"));
set.insert(String::from("."));
set.insert(String::from("e"));


fn lex_number(json: &mut String) -> Result<Option<String>, &'static str> {
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
	fn test_one() {
		let mut str = String::from("\"bytecode\"");
		assert_eq!(lex_string(&mut str).unwrap().unwrap(), "bytecode");
	}
    #[test]
	fn test_two() {
		let mut str = String::from("\"byte\"code\"");
		assert_eq!(lex_string(&mut str).unwrap().unwrap(), "byte");
	}

    #[test]
	fn test_three() {
		let mut str = String::from("");
		assert_eq!(lex_string(&mut str).unwrap(), None);
	}

    #[test]
	fn test_four() {
		let mut str = String::from("bananas");
		assert_eq!(lex_string(&mut str).unwrap(), None);
	}
}