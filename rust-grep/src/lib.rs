use std::error::Error;
use std::env;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

	for line in results {
		println!("{line}");
	}
	Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();

	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}
pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool
}

impl Config {
	pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {

		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Query not specified"),
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("Path not specified"),
		};

		let ignore_case: bool = if env::var("IGNORE_CASE").is_ok() {
			if env::var("IGNORE_CASE").unwrap().eq("1") {
				true
			} else{
				false
			}
		} else {
			false
		};


		Ok(Config {query, file_path, ignore_case})
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensetive() {
		let query = "Rust";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["Rust:"], search(query, contents));
	}

	#[test]
	fn search_insensitive() {
		let query = "pick";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["Pick three."], search_case_insensitive(query, contents));
	}
}