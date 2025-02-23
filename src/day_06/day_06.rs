use crate::space::point::Point2d;
use crate::space::space::Space2d;

type Instruction = (Action, Point2d, Point2d);

#[derive(Clone, Copy)]
enum Action {
	Toggle,
	Off,
	On
}

impl Action {
	fn part_1_modify (chosen_action: Action, current_value: isize) -> isize {
		return match chosen_action {
			Action::Off => 0,
			Action::On => 1,
			Action::Toggle => if current_value == 0 { 1 } else { 0 }
		};
	}

	fn part_2_modify (chosen_action: Action, current_value: isize) -> isize {
		let change = match chosen_action {
			Action::Toggle => 2,
			Action::On => 1,
			Action::Off => -1,
		};

		return isize::max(0, current_value + change);
	}
}

fn parse_input (input: &str) -> impl Iterator<Item = Instruction> + '_ {
	return input.lines()
		.map(|line| line.replace("turn", "").replace("through", ""))
		.map(|line| {
			// Couldn't be done in another map because .replace creates a String not a str
			// while .split_whitespace returns an iterator of &str causing ownership issues.
			let mut line = line.split_whitespace();
			let action = match line.next().unwrap() {
				"toggle" => Action::Toggle,
				"off" => Action::Off,
				"on" => Action::On,
				_ => panic!()
			};

			let start = Point2d::new_from_csv(line.next().unwrap());
			let end = Point2d::new_from_csv(line.next().unwrap());

			return (action, start, end);
		});
}

fn solve_problem (input: &str, modify_pixel: fn(Action, isize) -> isize) -> String {
	let instructions = parse_input(input);
	let mut lights = Space2d::new_grid_sized(0, 1000, 1000);
	for instruction in instructions {
		for point in Point2d::points_in_bounding_box(&instruction.1, &instruction.2) {
			let old_value = lights.get(&point).unwrap();
			let new_value = modify_pixel(instruction.0, *old_value);
			lights.set(&point, new_value);
		}
	}
	return lights.indices()
		.fold(0, |acc, x| acc + lights.get(&x).unwrap())
		.to_string();
}

pub fn day_06_1 (input: &str) -> String {
	return solve_problem(input, Action::part_1_modify);
}

pub fn day_06_2 (input: &str) -> String {
	return solve_problem(input, Action::part_2_modify);
}