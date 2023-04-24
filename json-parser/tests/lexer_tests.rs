#[cfg(test)]
mod tests {
	use jsonparser::lexer;

	#[test]
	fn test_one() {
		let json_string = "{\"name\":\"John\", \"age\":30, \"car\":null}".to_string();
		let res = lexer(json_string).unwrap();

		let items: Vec<String> = vec!["{", "name", ":", "John", ",", "age", ":", "30", ",", "car", ":", "null", "}"]
			.iter()
			.map(|s| s.to_string())
			.collect();

		assert_eq!(res, items);
	}

	/* 
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
    #[test]
	fn num_test_five() {
		let mut str = String::from("afjoijioa3");
		let set = create_numset();
		assert_eq!(lex_number(&mut str, &set).unwrap(), None);
	}

    #[test]
	fn bool_test_one() {
		let mut str = String::from("True123");
		assert_eq!(lex_bool(&mut str).unwrap().unwrap(), "True");
		assert_eq!(str, "123");
	}

    #[test]
	fn bool_test_two() {
		let mut str = String::from("False");
		assert_eq!(lex_bool(&mut str).unwrap().unwrap(), "False");
		assert_eq!(str, "");
	}

    #[test]
	fn bool_test_three() {
		let mut str = String::from("}");
		assert_eq!(lex_bool(&mut str).unwrap(), None);
		assert_eq!(str, "}");
	}

    #[test]
	fn bool_test_four() {
		let mut str = String::from("aghauTrue");
		assert_eq!(lex_bool(&mut str).unwrap(), None);
		assert_eq!(str, "aghauTrue");
	}

    #[test]
	fn bool_test_five() {
		let mut str = String::from("False741");
		assert_eq!(lex_bool(&mut str).unwrap().unwrap(), "False");
		assert_eq!(str, "741");
	}

    #[test]
	fn bool_test_six() {
		let mut str = String::from("null41");
		assert_eq!(lex_bool(&mut str).unwrap().unwrap(), "null");
		assert_eq!(str, "41");
	}

    #[test]
	fn lexer_test_one() {
		let str = String::from("{
			\"Fortnite\": 123}");
		let res = lexer(str).unwrap_or_else(|e| {
			println!("{}", e);
			exit(1);
		});
		println!("{:?}", res);
	}

    #[test]
	fn lexer_test_two() {
		let file_path = "joke.json";
		let file_contents = fs::read_to_string(file_path).unwrap();
		let res = lexer(file_contents).unwrap_or_else(|e| {
			println!("{}", e);
			exit(1);
		});
		println!("{:?}", res);
	}

    #[test]
	fn whitespace_test_one() {
		let mut str = String::from("\n");
		lex_whitespace(&mut str);
		assert_eq!(str, "");
	}
	*/
}