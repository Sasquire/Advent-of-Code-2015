use crate::graph::graph::Graph;

fn parse_input (input: &str) -> Graph<String, usize> {
	let line_iterator = input.lines()
		.map(|line| line.split(" ").collect::<Vec<_>>())
		.map(|line| (
			String::from(line[0]), // From
			String::from(line[2]), // To
			usize::from_str_radix(line[4], 10).unwrap() // Distance
		));
	let mut graph = Graph::new_from_iterator(line_iterator);
	graph.make_undirected();
	return graph;
}

pub fn day_09_1 (input: &str) -> String {
	let graph = parse_input(input);
	let mut best_distance = usize::MAX;
	for path in graph.all_paths() {
		let distance = graph.path_distance(path);
		if distance < best_distance {
			best_distance = distance;
		}
	}
	return best_distance.to_string();
}

pub fn day_09_2 (input: &str) -> String {
	let graph = parse_input(input);
	let mut best_distance = 0;
	for path in graph.all_paths() {
		let distance = graph.path_distance(path);
		if distance > best_distance {
			best_distance = distance;
		}
	}
	return best_distance.to_string();
}