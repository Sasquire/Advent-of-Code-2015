fn first_n_hex_are_0 (n: usize, input: String) -> bool {
	let result = md5::compute(input);

	for i in 0..n {
		let hex_at_index = if i % 2 == 0 {
			(0b11110000 & result[i/2]) >> 4
		} else {
			(0b00001111 & result[i/2]) >> 0
		};
		if hex_at_index != 0 { return false; }
	}

	return true;
}

fn solve_problem (n: usize, input: &str) -> String {
	for i in 1.. {
		let seed = format!("{}{}", input, i);
		if first_n_hex_are_0(n, seed) {
			return i.to_string();
		}
	}

	panic!();
}

pub fn day_04_1 (input: &str) -> String {
	return solve_problem(5, input);
}

pub fn day_04_2 (input: &str) -> String {
	return solve_problem(6, input);
}
