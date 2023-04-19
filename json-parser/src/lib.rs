use std:: {
	collections::HashSet, 
};

//fn lexer(json: String) -> Vec<String> {
	//let tokens: Vec<String> = Vec::new();
//}

fn create_numset() -> HashSet<String> {
	let mut set = HashSet::new();
	for i in 0..10 {
		set.insert(i.to_string());
	}
	set.insert(String::from("-"));
	set.insert(String::from("."));
	set.insert(String::from("e"));
	set
}

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

fn lex_number(json: &mut String, set: &HashSet<String>) -> Result<Option<String>, &'static str> {
	if !json.starts_with(|c: char| set.contains(&c.to_string())) {
		return Ok(None);
	}
	let mut out: String = String::new();
	while json.len() > 0 {
		if !json.starts_with(|c: char| set.contains(&c.to_string())) {
			return Ok(Some(out));
		} else {
			out.push(json.remove(0));
		}
	}
	return Err("invalid json");
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
	fn string_test_one() {
		let mut str = String::from("\"bytecode\"");
		assert_eq!(lex_string(&mut str).unwrap().unwrap(), "bytecode");
	}
    #[test]
	fn string_test_two() {
		let mut str = String::from("\"byte\"code\"");
		assert_eq!(lex_string(&mut str).unwrap().unwrap(), "byte");
	}

    #[test]
	fn string_test_three() {
		let mut str = String::from("");
		assert_eq!(lex_string(&mut str).unwrap(), None);
	}

    #[test]
	fn string_test_four() {
		let mut str = String::from("\"jfj1518\"");
		assert_eq!(lex_string(&mut str).unwrap().unwrap(), "jfj1518");
	}

    #[test]
	fn num_test_one() {
		let mut str = String::from("481ajgo24");
		let set = create_numset();
		assert_eq!(lex_number(&mut str, &set).unwrap().unwrap(), "481");
	}

    #[test]
	fn num_test_two() {
		let mut str = String::from("a353");
		let set = create_numset();
		assert_eq!(lex_number(&mut str, &set).unwrap(), None);
	}

    #[test]
	fn num_test_three() {
		let mut str = String::from("15.135315135afa");
		let set = create_numset();
		assert_eq!(lex_number(&mut str, &set).unwrap().unwrap(), "15.135315135");
	}

    #[test]
	fn num_test_four() {
		let mut str = String::from("-5.4170e14ag");
		let set = create_numset();
		assert_eq!(lex_number(&mut str, &set).unwrap().unwrap(), "-5.4170e14");
	}
}