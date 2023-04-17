use std::env;
use std::process;
use rust_grep::Config;

fn main() {
	let config = Config::build(env::args()).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		process::exit(1);
	});

	if let Err(e) = rust_grep::run(config) {
		println!("{e}");

		process::exit(1);
	}
}
