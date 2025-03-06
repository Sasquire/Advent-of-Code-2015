use crate::space::point::Point2d;
use std::collections::HashSet;

use itertools::Itertools;

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
	let (flesh_santa_route, robot_santa_route) = parse_input(input)
		.chunks(2)
		.into_iter()
		.map(|mut row| (row.next().unwrap(), row.next().unwrap()))
		.unzip::<Point2d, Point2d, Vec<_>, Vec<_>>();

	let mut all_houses = HashSet::new();
	all_houses.extend(houses_visited(flesh_santa_route.into_iter()));
	all_houses.extend(houses_visited(robot_santa_route.into_iter()));

	return all_houses.len().to_string();
}
