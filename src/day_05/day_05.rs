use std::char;

fn parse_input (input: &str) -> impl Iterator<Item = Vec<char>> + '_ {
	return input.lines().map(|x| x.chars().collect::<Vec<_>>())
}

fn count_vowels (line: &Vec<char>) -> usize {
	let vowels = ['a', 'e', 'i', 'o', 'u'];
	
	return line.iter()
		.map(|x| if vowels.contains(x) { 1 } else { 0 })
		.sum();
}

fn contains_letter_pair (line: &Vec<char>) -> bool {
	for i in 1..line.len() {
		if line[i - 1] == line[i] { return true; }
	}
	return false;
}

fn contains_bad_string (line: &Vec<char>) -> bool {
	let bad_strings = ["ab", "cd", "pq", "xy"];
	let line = line.iter().collect::<String>();
	return bad_strings.iter().any(|x| line.contains(x));
}

pub fn day_05_1 (input: &str) -> String {
	return parse_input(input)
		.filter(|x| contains_bad_string(x) == false)
		.filter(|x| contains_letter_pair(x) == true)
		.filter(|x| 3 <= count_vowels(x))
		.count()
		.to_string();
}

fn has_double_no_overlap (line: &Vec<char>) -> bool {
	for i in 0..line.len() - 3 {
		for j in (i+2)..line.len() - 1 {
			if line[i] == line[j] && line[i + 1] == line[j + 1] {
				return true;
			}
		}
	}
	return false;
}

fn has_repeat_with_gap (line: &Vec<char>) -> bool {
	for i in 2..line.len() {
		if line[i] == line[i - 2] {
			return true;
		}
	}
	return false;
}

pub fn day_05_2 (input: &str) -> String {
	return parse_input(input)
		.filter(|x| has_repeat_with_gap(x) == true)
		.filter(|x| has_double_no_overlap(x) == true)
		.count()
		.to_string();
}
