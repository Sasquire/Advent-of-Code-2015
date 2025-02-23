use crate::space::point::Point2d;

pub struct Space2d<Star> {
	data: Vec<Vec<Star>>,

	x_len: usize,
	y_len: usize,

	x_offset: isize,
	y_offset: isize
}

impl<Star: std::clone::Clone> Space2d<Star> {
	#[allow(dead_code)]

	pub fn new_grid_sized (default_object: Star, num_rows: usize, num_cols: usize) -> Space2d<Star> {
		return Space2d {
			data: vec![vec![default_object; num_cols]; num_rows],
			x_len: num_cols,
			y_len: num_rows,

			x_offset: 0,
			y_offset: 0
		};
	}
}

impl Space2d<String> {
	pub fn new_from_str (input: &str) -> Space2d<String> {
		let data = input.lines()
			.map(|row| row.chars().map(|x| x.to_string()).collect::<Vec<_>>())
			.collect::<Vec<_>>();

		return Space2d {
			x_len: data[0].len(),
			y_len: data.len(),

			x_offset: 0,
			y_offset: 0,

			data: data
		};
	}
}

impl<Star> Space2d<Star> {
	#[allow(dead_code)]

	// get 1d vectors from different things
	// get row, get col, get height, get xxx

	// insert vectors that nudge like things to the side
	// insert row, col, height, etc

	// remove row, col, etc, etc

	// rotate and reflect

	pub fn indices (&self) -> impl Iterator<Item = Point2d> + '_ {
		(self.y_offset..(self.y_offset + self.y_len as isize)).flat_map(move |y|
			(self.x_offset..(self.x_offset + self.x_len as isize)).map(move |x|
				Point2d::new(x, y)
			)
		)
	}

	pub fn point_is_in_bounds (&self, point: &Point2d) -> bool {
		return true
			&& self.x_offset <= point.x() && point.x() < self.x_offset + self.x_len as isize
			&& self.y_offset <= point.y() && point.y() < self.y_offset + self.y_len as isize;
	}

	pub fn get (&self, point: &Point2d) -> Option<&Star> {
		if self.point_is_in_bounds(point) {
			Some(&self.data[(point.y() + self.y_offset) as usize][(point.x() + self.x_offset) as usize])
		} else {
			None
		}
	}

	pub fn set (&mut self, point: &Point2d, value: Star) {
		if self.point_is_in_bounds(point) {
			self.data[(point.y() + self.y_offset) as usize][(point.x() + self.x_offset) as usize] = value;
		} else {
			// Do nothing
		}
	}

	pub fn num_cols (&self) -> usize { self.x_len }
	pub fn num_rows (&self) -> usize { self.y_len }
	// pub fn x_offset (&self) -> usize { self.x_offset }
	// pub fn y_offset (&self) -> usize { self.y_offset }
}

impl<Star: std::cmp::PartialEq> Space2d<Star> {
	#[allow(dead_code)]

	pub fn find_all <'a> (&'a self, needle: &'a Star) -> impl Iterator<Item = Point2d> + 'a {
		return self.indices().filter(move |x| self.get(x).unwrap() == needle);
	}
}

impl<Star: std::string::ToString> Space2d<Star> {
	#[allow(dead_code)]

	pub fn print_compact (&self) {
		for row in self.y_offset..(self.y_offset + self.y_len as isize) {
			for col in self.x_offset..(self.x_offset + self.x_len as isize) {
				print!("{}", self.get(&Point2d::new(col, row)).unwrap().to_string().chars().nth(0).unwrap());
			}
			println!();
		}
	}
}