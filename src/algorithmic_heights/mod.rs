use std::cmp::{Eq, Ord, PartialOrd};
use std::hash::Hash;
use std::marker::Copy;
use std::collections::{HashMap, HashSet};

pub fn fibo(n: u32) -> u32 {
	let mut i = 0;
	let mut j = 1;
	for _ in 0..n {
		let tmp = i;
		i = j;
		j = tmp + j;
	}
	i
}

fn binary_search<T: Ord + Copy>(sorted_arr: &[T], n: T) -> i32 {
	let mut i = 0;
	let mut j = sorted_arr.len()- 1;
	let mut m = 0;

	// check if it's inside the array
	if n < sorted_arr[i] || n > sorted_arr[j] {
		return -1;
	}

	// check the left and right bounds
	if n == sorted_arr[i] {
		return i as i32;
	} else if n == sorted_arr[j] {
		return j as i32;
	}

	loop {
		let tmp = (i + j) / 2;
		if tmp == m {
			return -1; // not found
		}
		m = tmp;

		let val = sorted_arr[m];
		if val == n {
			return m as i32;
		} else if val > n {
			j = m;
		} else if val < n {
			i = m;
		}
	}
}

pub fn bins<T: Ord + Copy>(sorted_arr: &[T], n: T) -> i32 {
	// add one to get 1 based index
	match binary_search(sorted_arr, n) {
		-1 => -1,	// preserve -1
		x  => x + 1,
	}
}

fn create_graph<T: Eq + Hash + Ord + Copy>(edge_list: &[Vec<T>]) -> HashMap<T, HashSet<T>> {
	let mut graph: HashMap<T, HashSet<T>> = HashMap::with_capacity(edge_list.len());
	for edge in edge_list {
		let a = edge[0];
		let b = edge[1];

		if graph.contains_key(&a) {
			if let Some(set) = graph.get_mut(&a) {
				set.insert(b);
			};
		} else {
			let mut set = HashSet::new();
			set.insert(b);
			graph.insert(a, set);
		}

		if graph.contains_key(&b) {
			if let Some(set) = graph.get_mut(&b) {
				set.insert(a);
			};
		} else {
			let mut set = HashSet::new();
			set.insert(a);
			graph.insert(b, set);
		}
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

pub fn ins<T: PartialOrd + Copy>(array: &mut [T]) -> usize {
	let mut result = 0;
	for i in 1..array.len() {
		let mut k = i;
		while k > 0 && array[k] < array[k - 1] {
			result += 1;
			let tmp = array[k];
			array[k] = array[k - 1];
			array[k - 1] = tmp;
			k -= 1;
		}
	}
	return result;
}