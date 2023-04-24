use std:: {
	collections::HashSet, 
};

pub fn lexer(json: String) -> Result<Vec<String>, &'static str> {
	let mut json = json.clone();
	let special_chars = create_special();
	let num_chars = create_numset();
	let mut tokens: Vec<String> = Vec::new();
	while json.len() > 0 {
		lex_whitespace(&mut json );
		match lex_string(&mut json)? {
			None => {},
			Some(parsed) => {
				tokens.push(parsed);
			}
		}
		match lex_number(&mut json, &num_chars)? {
			None => {},
			Some(parsed) => {
				tokens.push(parsed);
			}
		}
		match lex_bool(&mut json)? {
			None => {},
			Some(parsed) => {
				tokens.push(parsed);
			}
		}
		match lex_special(&mut json, &special_chars)? {
			None => {},
			Some(parsed) => {
				tokens.push(parsed);
			}
		}
	}
	return Ok(tokens);
}

fn create_special() -> HashSet<String> {
	let mut set = HashSet::new();

	//list of special chars
	vec!["[", "]", "{", "}", ",", ":"]
		.iter()
		.map(|c| c.to_string())
		.for_each(|c| {
			set.insert(c);
		});

	set
}

fn create_numset() -> HashSet<String> {
	let mut set = HashSet::new();
	for i in 0..10 {
		set.insert(i.to_string());
	}
	
	//special chars
	vec!["-", ".", "e"]
		.iter()
		.map(|c| c.to_string())
		.for_each(|c| {
			set.insert(c);
		});

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
			json.remove(0);
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


fn lex_bool(json: &mut String) -> Result<Option<String>, &'static str> {
	if json.len() < 4 {
		return Ok(None);
	}
	if &json[0..4] == "true" {
		let out: String = json.drain(..4).collect();
		return Ok(Some(out));
	}

	if &json[0..5] == "false" {
		let out: String = json.drain(..5).collect();
		return Ok(Some(out));
	}

	if &json[0..4] == "null" {
		let out: String = json.drain(..4).collect();
		return Ok(Some(out));
	}
	return Ok(None);
}

fn lex_special(json: &mut String, set: &HashSet<String>) -> Result<Option<String>, &'static str> {
	if !json.starts_with(|c: char| set.contains(&c.to_string())) {
		return Ok(None);
	}
	return Ok(Some(json.remove(0).to_string()));
}

fn lex_whitespace(json: &mut String) {
	while json.starts_with(|c: char| {
		let c = c.to_string();
		if c == " " {
			return true;
		}
		if c == "\n" {
			return true;
		}
		if c == "\t" {
			return true;
		}
		return false;
	}) {
		json.remove(0);
	}
}
