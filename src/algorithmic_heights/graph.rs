use std::hash::Hash;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn create_graph<T: Eq + Hash + Ord + Copy>(edge_list: &[Vec<T>]) -> HashMap<T, HashSet<T>> {
	let mut graph: HashMap<T, HashSet<T>> = HashMap::with_capacity(edge_list.len());
	for edge in edge_list {
		let a = edge[0];
		let b = edge[1];

		graph.entry(a).or_insert(HashSet::new()).insert(b);
		graph.entry(b).or_insert(HashSet::new()).insert(a);
	}
	return graph;
}

pub fn create_digraph<T: Eq + Hash + Ord + Copy>(edge_list: &[Vec<T>]) -> HashMap<T, HashSet<T>> {
	let mut graph: HashMap<T, HashSet<T>> = HashMap::with_capacity(edge_list.len());
	for edge in edge_list {
		let a = edge[0];
		let b = edge[1];

		graph.entry(a).or_insert(HashSet::new()).insert(b);
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

pub fn bfs(graph: &HashMap<usize, HashSet<usize>>, num_nodes: usize) -> Vec<i32> {
	let mut result: Vec<i32> = std::iter::repeat(-1).take(num_nodes).collect();

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();

	let start = 1;
	queue.push_back((start, 0));
	visited.insert(start);

	while !queue.is_empty() {
		let (node, dist) = queue.pop_front().unwrap();
		result[node - 1] = dist;

		if let Some(set) = graph.get(&node) {
			for next in set {
				if !visited.contains(next) {
					visited.insert(*next);
					queue.push_back((*next, dist + 1));
				}
			}
		}
	}

	return result;
}