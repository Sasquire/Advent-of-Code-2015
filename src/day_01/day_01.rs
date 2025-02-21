fn parse_input (input: &str) -> impl Iterator<Item = isize> + '_ {
	return input.chars()
		.map(|x| match x {
			'(' =>  1,
			')' => -1,
			_ => 0 
		});
}

pub fn day_01_1 (input: &str) -> String {
	return parse_input(input).fold(0, |acc, e| acc + e).to_string();
}

pub fn day_01_2 (input: &str) -> String {
	let mut current_floor = 0;
	for (index, direction) in parse_input(input).enumerate() {
		if current_floor == -1 {
			return index.to_string();
		} else {
			current_floor += direction;
		}
	}
	return 0.to_string();
}
