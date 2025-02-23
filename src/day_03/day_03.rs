use crate::space::point::Point2d;
use std::collections::HashSet;

fn parse_input (input: &str) -> impl Iterator<Item = Point2d> + '_ {
	return input.chars().map(|x| match x {
		'^' => Point2d::cardinal_north(),
		'>' => Point2d::cardinal_east(),
		'v' => Point2d::cardinal_south(),
		'<' => Point2d::cardinal_west(),
		_ => panic!()
	});
}

fn houses_visited (directions: impl Iterator<Item = Point2d>) -> HashSet<Point2d> {
	return std::iter::once(Point2d::origin()) // Direction of not moving
		.chain(directions)
		.scan(Point2d::origin(), |santas_position, direction| {
			*santas_position = *santas_position + direction;
			Some(santas_position.clone())
		})
		.collect::<HashSet<Point2d>>();
}

pub fn day_03_1 (input: &str) -> String {
	return houses_visited(parse_input(input)).len().to_string();
}

pub fn day_03_2 (input: &str) -> String {
	let input = parse_input(input).collect::<Vec<_>>();
	let mut flesh_santa_route = vec![];
	let mut robot_santa_route = vec![];

	for i in (0..input.len()).step_by(2) {
		flesh_santa_route.push(input[i + 0]);
		robot_santa_route.push(input[i + 1]);
	}

	let mut all_houses = HashSet::new();
	all_houses.extend(houses_visited(flesh_santa_route.into_iter()));
	all_houses.extend(houses_visited(robot_santa_route.into_iter()));

	return all_houses.len().to_string();
}
