struct Present {
	length: usize,
	width: usize,
	height: usize
}

impl Present {
	pub fn new (length: usize, width: usize, height: usize) -> Present {
		return Present { length, width, height };
	}

	pub fn paper (&self) -> usize {
		let top_bottom = 2 * self.length * self.width;
		let port_starboard = 2 * self.length * self.height;
		let bow_stern = 2 * self.width * self.height;
	
		let slack = if top_bottom <= port_starboard && top_bottom <= bow_stern {
			top_bottom / 2
		} else if port_starboard <= top_bottom && port_starboard <= bow_stern {
			port_starboard / 2
		} else if bow_stern <= port_starboard && bow_stern <= top_bottom {
			bow_stern / 2
		} else {
			panic!();
		};
	
		return top_bottom + port_starboard + bow_stern + slack;
	}

	pub fn ribbon (&self) -> usize {
		let bow_requirements = self.length * self.width * self.height;

		let shortest_perimeter = if self.length <= self.height && self.width <= self.height {
			2 * self.length + 2 * self.width
		} else if self.length <= self.width && self.height <= self.width {
			2 * self.length + 2 * self.height
		} else if self.width <= self.length && self.height <= self.length {
			2 * self.width + 2 * self.height
		} else {
			panic!();
		};
	
		return bow_requirements + shortest_perimeter;
	}
}

fn parse_input (input: &str) -> impl Iterator<Item = Present> + '_ {
	return input.lines()
		.map(|row| row.split("x")
			.map(|e| usize::from_str_radix(e, 10).unwrap())
			.collect::<Vec<_>>()
		)
		.map(|row| Present::new(row[0], row[1], row[2]));
}

pub fn day_02_1 (input: &str) -> String {
	return parse_input(input)
		.map(|x| x.paper())
		.fold(0, |acc, e| acc + e)
		.to_string();
}

pub fn day_02_2 (input: &str) -> String {
	return parse_input(input)
		.map(|x| x.ribbon())
		.fold(0, |acc, e| acc + e)
		.to_string();
}