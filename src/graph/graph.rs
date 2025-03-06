use itertools::Itertools;
use std::collections::HashMap;

trait LocationTraits: std::cmp::Eq
	+ std::hash::Hash
	+ std::clone::Clone
	+ std::fmt::Debug
	+ std::fmt::Display {}
impl<T: std::cmp::Eq
	+ std::hash::Hash
	+ std::clone::Clone
	+ std::fmt::Debug
	+ std::fmt::Display> LocationTraits for T {}

trait DistanceTraits: std::default::Default + std::clone::Clone + std::ops::Add + std::cmp::Ord {}
impl<T: std::default::Default + std::clone::Clone+ std::ops::Add + std::cmp::Ord> DistanceTraits for T {}

#[derive(Debug, Clone)]
pub struct Graph<L, D> {
	distances: HashMap<[L; 2], D>,

	is_directed: bool
}

impl<Location, Distance> Graph<Location, Distance>
where
	Location: LocationTraits,
	Distance: DistanceTraits<Output = Distance>
{
	pub fn new_empty () -> Graph<Location, Distance> {
		return Graph {
			distances: HashMap::new(),
			is_directed: true
		};
	}

	pub fn new_from_iterator (iterator: impl Iterator<Item = (Location, Location, Distance)>) -> Graph<Location, Distance> {
		let mut graph = Graph::new_empty();
		iterator.for_each(|x| graph.add_path(x.0, x.1, x.2));
		return graph;
	}

	pub fn make_directed (&mut self) { self.is_directed = true; }
	pub fn make_undirected (&mut self) { self.is_directed = false; }

	pub fn add_path (&mut self, from: Location, to: Location, distance: Distance) {
		self.distances.insert([from, to], distance);
	}

	pub fn get_locations (&self) -> impl Iterator<Item = &Location> {
		return self.distances.keys().flatten().unique();
	}

	pub fn path_distance (&self, path_keys: Vec<&Location>) -> Distance {
		let mut total_distance = Distance::default();
		for i in 1..path_keys.len() {
			let from = path_keys[i - 1].clone();
			let to = path_keys[i].clone();

			let forward = self.distances.get(&[from.clone(), to.clone()]);
			let backward = self.distances.get(&[to.clone(), from.clone()]);

			total_distance = total_distance + if self.is_directed {
				forward.unwrap().clone()
			} else {
				forward.or_else(|| backward).unwrap().clone()
			};
		}
		
		return total_distance;
	}

	pub fn all_paths (&self) -> impl Iterator<Item = Vec<&Location>> {
		return self.get_locations().permutations(self.get_locations().count());
	}
}