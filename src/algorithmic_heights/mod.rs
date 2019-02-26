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
	return i;
}

fn binary_search<T: Ord + Copy>(sorted_arr: &[T], n: T) -> Option<usize> {
	let mut i = 0;
	let mut j = sorted_arr.len() - 1;
	let mut m = 0;

	// check if it's inside the array
	if n < sorted_arr[i] || n > sorted_arr[j] {
		return None;
	}

	// check the left and right bounds
	if n == sorted_arr[i] {
		return Some(i);
	} else if n == sorted_arr[j] {
		return Some(j);
	}

	loop {
		let tmp = (i + j) / 2;
		if tmp == m {
			return None;
		}
		m = tmp;

		let val = sorted_arr[m];
		if val == n {
			return Some(m);
		} else if val > n {
			j = m;
		} else if val < n {
			i = m;
		}
	}
}

pub fn bins<T: Ord + Copy>(sorted_arr: &[T], nums: &[T]) -> Vec<i32> {

	// add one to get 1 based index
	let add_one = |index| -> i32 {
		match index {
			None => -1, // preserve -1
			Some(x) => (x as i32) + 1,
		}
	};

	let result = nums.into_iter()
					 .map(|n| add_one(binary_search(sorted_arr, *n)))
					 .collect();
	return result;
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

pub fn maj<T: Eq + Hash + Copy>(input_arr: &[Vec<T>], default: T) -> Vec<T> {
	let mut result = vec![];
	for array in input_arr {
		let mut map = HashMap::new();
		for val in array {
			if map.contains_key(&val) {
				if let Some(count) = map.get_mut(&val) {
					*count += 1;
				}
			} else {
				map.insert(val, 1);
			}
		}

		let half = array.len() / 2;
		let mut found = false;

		for (key, count) in map {
			if count > half {
				found = true;
				result.push(*key);
				break;
			}
		}

		if !found {
			result.push(default);
		}
	}
	return result;
}

pub fn mer<T: Copy + Ord>(array_a: &[T], array_b: &[T]) -> Vec<T> {
	let mut i = 0;
	let mut j = 0;

	let total_len = array_a.len() + array_b.len();
	let mut result = Vec::with_capacity(total_len);

	while result.len() < total_len {
		if j >= array_b.len() || array_a[i] < array_b[j] {
			result.push(array_a[i]);
			i += 1;
		} else if i >= array_a.len() || array_a[i] > array_b[j] {
			result.push(array_b[j]);
			j += 1;
		} else {
			result.push(array_a[i]);
			result.push(array_b[j]);
			i += 1;
			j += 1;
		}
	}

	return result;
}