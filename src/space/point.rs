const DIMENSIONS_2D_SIZE: usize = 2;

const DIMENSIONS_2D_ORIGIN: Point2d = Point2d { coordinates: [ 0, 0] };
const DIMENSIONS_2D_DIRECTION_EAST: Point2d = Point2d { coordinates: [ 1, 0] };
const DIMENSIONS_2D_DIRECTION_WEST: Point2d = Point2d { coordinates: [-1, 0] };
const DIMENSIONS_2D_DIRECTION_NORTH: Point2d = Point2d { coordinates: [0,  1] };
const DIMENSIONS_2D_DIRECTION_SOUTH: Point2d = Point2d { coordinates: [0, -1] };

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point2d {
	coordinates: [isize; DIMENSIONS_2D_SIZE]
}

impl Point2d {
	const fn dimensions () -> usize { DIMENSIONS_2D_SIZE }
	pub fn dimension_value (&self, dimension: usize) -> isize { self.coordinates[dimension] }

	pub fn x (&self) -> isize { self.coordinates[0] }
	pub fn y (&self) -> isize { self.coordinates[1] }
}

// Took great inspiration from ndarray's macro
// https://docs.rs/ndarray/latest/src/ndarray/impl_ops.rs.html#53
macro_rules! impl_elementwise_operation {
	($type1:ty, $type2:ty, $std_ops_name:ident, $fn_name:ident, $operator:tt) => {
		
		// Both owned
		impl std::ops::$std_ops_name<$type2> for $type1 {
			type Output = $type1;

			fn $fn_name (self, other: $type2) -> $type1 {
				let mut coordinates = [0; Self::Output::dimensions()];
				for i in 0..Self::Output::dimensions() {
					coordinates[i] = self.dimension_value(i) $operator other.dimension_value(i);
				}
				Self::Output { coordinates }
			}
		}

		// lhs is reference
		impl<'a> std::ops::$std_ops_name<$type2> for &'a $type1 {
			type Output = $type1;

			fn $fn_name (self, other: $type2) -> $type1 {
				let mut coordinates = [0; Self::Output::dimensions()];
				for i in 0..Self::Output::dimensions() {
					coordinates[i] = self.dimension_value(i) $operator other.dimension_value(i);
				}
				Self::Output { coordinates }
			}
		}

		// rhs is reference
		impl<'b> std::ops::$std_ops_name<&'b $type2> for $type1 {
			type Output = $type1;

			fn $fn_name (self, other: &'b $type2) -> $type1 {
				let mut coordinates = [0; Self::Output::dimensions()];
				for i in 0..Self::Output::dimensions() {
					coordinates[i] = self.dimension_value(i) $operator other.dimension_value(i);
				}
				Self::Output { coordinates }
			}
		}

		// Both references
		impl<'a, 'b> std::ops::$std_ops_name<&'b $type2> for &'a $type1 {
			type Output = $type1;

			fn $fn_name (self, other: &'b $type2) -> $type1 {
				let mut coordinates = [0; Self::Output::dimensions()];
				for i in 0..Self::Output::dimensions() {
					coordinates[i] = self.dimension_value(i) $operator other.dimension_value(i);
				}
				Self::Output { coordinates }
			}
		}
	}
}

pub trait FakeDimensions {
	fn dimension_value (self, dimension: usize) -> Self;
}

impl FakeDimensions for isize {
	fn dimension_value (self, _dimension: usize) -> Self { self }
}

impl_elementwise_operation!(Point2d, isize, Add, add, +);
impl_elementwise_operation!(Point2d, isize, Sub, sub, -);
impl_elementwise_operation!(Point2d, isize, Mul, mul, *);
impl_elementwise_operation!(Point2d, isize, Div, div, /);
impl_elementwise_operation!(Point2d, isize, Rem, rem, %);

impl_elementwise_operation!(Point2d, Point2d, Add, add, +);
impl_elementwise_operation!(Point2d, Point2d, Sub, sub, -);

impl Point2d {
	pub fn origin () -> Point2d { DIMENSIONS_2D_ORIGIN }
	pub fn cardinal_north () -> Point2d { DIMENSIONS_2D_DIRECTION_NORTH }
	pub fn cardinal_east () -> Point2d { DIMENSIONS_2D_DIRECTION_EAST }
	pub fn cardinal_south () -> Point2d { DIMENSIONS_2D_DIRECTION_SOUTH }
	pub fn cardinal_west () -> Point2d { DIMENSIONS_2D_DIRECTION_WEST }
}

impl Point2d {
	#[allow(dead_code)]

	pub fn new (x: isize, y: isize) -> Point2d {
		Point2d { coordinates: [x, y] }
	}

	pub fn new_from_csv (input: &str) -> Point2d {
		let mut separated = input.split(",").map(|y| isize::from_str_radix(y, 10).unwrap());
		let x = separated.next().unwrap();
		let y = separated.next().unwrap();
		return Point2d { coordinates: [x, y] };
	}

	// Higher dimensions include up, down, ana, kata
	pub fn north (&self) -> Point2d { self + DIMENSIONS_2D_DIRECTION_NORTH }
	pub fn east (&self) -> Point2d { self + DIMENSIONS_2D_DIRECTION_EAST }
	pub fn south (&self) -> Point2d { self + DIMENSIONS_2D_DIRECTION_SOUTH }
	pub fn west (&self) -> Point2d { self + DIMENSIONS_2D_DIRECTION_WEST }

	pub fn neighbor_cross (&self) -> impl Iterator<Item = Point2d> + '_ {
		return vec![self.north(), self.east(), self.west(), self.south()].into_iter();
	}

	pub fn neighbor_block (&self) -> impl Iterator<Item = Point2d> + '_ {
		return vec![
			self.north().west(), self.north(), self.north().east(),
			self.west(),                       self.east(),
			self.south().west(), self.south(), self.south().east()
		].into_iter();
	}
}

impl Point2d {
	#[allow(dead_code)]

	pub fn points_in_bounding_box <'a> (start: &'a Point2d, end: &'a Point2d) -> impl Iterator<Item = Point2d> + 'a {
		(start.y()..=end.y()).flat_map(move |y|
			(start.x()..=end.x()).map(move |x|
				Point2d::new(x, y)
			)
		)
	}
}

// https://stackoverflow.com/questions/77588838/how-to-create-a-custom-hash-function-in-rust
impl std::hash::Hash for Point2d {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let x = (self.x().unsigned_abs() * 2 + (self.x().signum() - 1).unsigned_abs() / 2) as u64;
		let y = (self.y().unsigned_abs() * 2 + (self.y().signum() - 1).unsigned_abs() / 2) as u64;

		/* szudziks function */
		let hash_val = if x >= y { x * x + x + y } else { x + y * y };
		state.write_u64(hash_val);
	}
}

impl std::fmt::Display for Point2d {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "({}, {})", self.x(), self.y())
	}
}