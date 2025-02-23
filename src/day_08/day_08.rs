use regex::Regex;

pub fn day_08_1 (input: &str) -> String {
	return input.lines()
		.map(|line| {
			let code_length = line.len();
			let line = Regex::new(r#"(\\\\|\\x..|\\")"#).unwrap().replace_all(&line, "x");
			let parsed_length = line.len() - 2;

			return (code_length, parsed_length);
		})
		.fold(0, |acc, e| acc + (e.0.abs_diff(e.1)))
		.to_string();
}

pub fn day_08_2 (input: &str) -> String {
	return input.lines()
		.map(|line| {
			let code_length = line.len();
			let line = line.replace("\"", "\"\"").replace("\\", "\\\\");
			let parsed_length = line.len() + 2;

			return (code_length, parsed_length);
		})
		.fold(0, |acc, e| acc + (e.0.abs_diff(e.1)))
		.to_string();
}