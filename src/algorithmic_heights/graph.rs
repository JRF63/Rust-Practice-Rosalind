use std::hash::Hash;
use std::collections::{HashMap, HashSet};

fn create_graph<T: Eq + Hash + Ord + Copy>(edge_list: &[Vec<T>]) -> HashMap<T, HashSet<T>> {
	let mut graph: HashMap<T, HashSet<T>> = HashMap::with_capacity(edge_list.len());
	for edge in edge_list {
		let a = edge[0];
		let b = edge[1];

		graph.entry(a).or_insert(HashSet::new()).insert(b);
		graph.entry(b).or_insert(HashSet::new()).insert(a);
	}
	return graph;
}

pub fn deg(input_arr: &[Vec<u32>]) -> Vec<usize> {
	let num_nodes = input_arr[0][0];
	let graph = create_graph(&input_arr[1..]);
	let mut result = vec![];
	for key in 1..(num_nodes + 1) {
		match graph.get(&key) {
			Some(set) => result.push(set.len()),
			None => result.push(0),
		}
	}
	return result;
}

pub fn ddeg(input_arr: &[Vec<u32>]) -> Vec<usize> {
	let num_nodes = input_arr[0][0];
	let graph = create_graph(&input_arr[1..]);
	let mut result = vec![];
	for key in 1..(num_nodes + 1) {
		match graph.get(&key) {
			Some(neighborhood) => {
						let mut deg = 0;
						for neighbor in neighborhood {
							if let Some(set) = graph.get(&neighbor) {
								deg += set.len();
							}
						}
						result.push(deg);
					},
			None => result.push(0),
		}
	}
	return result;
}