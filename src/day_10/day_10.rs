use itertools::Itertools;

fn parse_input<'a> (input: &'a str) -> Box<dyn Iterator<Item = usize> + 'a> {
	return Box::new(input.chars().map(|x| usize::from_str_radix(x.to_string().as_str(), 10).unwrap()));
}

fn look_and_say<'b> (input: impl Iterator<Item = usize> + 'b) -> Box<dyn Iterator<Item = usize> + 'b> {
	return Box::new(input.into_iter()
		.dedup_with_count()
		.map(|(count, value)| [count, value])
		.flatten());
}

fn solve (input: &str, count: usize) -> usize {
	let mut input = parse_input(input);
	for i in 0..count {
		input = look_and_say(input);
	}
	return input.count();
}

pub fn day_10_1 (input: &str) -> String {
	println!("{} = {}", input, solve("11", 1));
	return solve(input, 40).to_string();
}

pub fn day_10_2 (input: &str) -> String {
	return solve(input, 50).to_string();
}