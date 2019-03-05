use std::hash::Hash;
use std::collections::{HashMap, HashSet};

pub mod fibo;
pub mod bins;

pub mod threesum;


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


fn merge_to_tmp<T: Copy + Ord>(array_a: &[T], array_b: &[T], tmp: &mut [T]) {
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;

	let total_len = array_a.len() + array_b.len();

	while k < total_len {
		if j >= array_b.len() {
			tmp[k] = array_a[i];
			i += 1;
		} else if i >= array_a.len() {
			tmp[k] = array_b[j];
			j += 1;
		} else if array_a[i] < array_b[j] {
			tmp[k] = array_a[i];
			i += 1;
		} else if array_a[i] > array_b[j] {
			tmp[k] = array_b[j];
			j += 1;
		} else {
			tmp[k] = array_a[i];
			i += 1;
			k += 1;
			tmp[k] = array_b[j];
			j += 1;
		}
		k += 1;
	}
}

pub fn mer<T: Copy + Ord>(array_a: &[T], array_b: &[T]) -> Vec<T> {
	let mut result = vec![array_a[0]; array_a.len() + array_b.len()];
	merge_to_tmp(array_a, array_b, &mut result);
	return result;
}

pub fn ms<T: Copy + Ord>(array: &mut [T]) {
	let mut tmp: Vec<T> = array.to_vec();

	let mut sort_size = 1;

	let mut n = array.len();
	while n > 0 {
		n = n / 2;

		let next_size = sort_size * 2;

		let quot = array.len() / next_size;

		let mut i = 0;

		for _ in 0..quot {
			let j = i + sort_size;
			let k = j + sort_size;
			merge_to_tmp(&array[i..j], &array[j..k], &mut tmp[i..k]);
			i += next_size;
		}

		if i + sort_size < array.len() {
			let j = i + sort_size;
			let k = array.len();
			merge_to_tmp(&array[i..j], &array[j..k], &mut tmp[i..k]);
		}

		sort_size = next_size;
		array.copy_from_slice(&tmp);
	}
}

pub fn twosum(arrays: &[Vec<i32>]) -> Vec<Option<(i32, i32)>> {
	let mut result = vec![];
	'outer: for array in arrays {
		let mut map = HashMap::new();

		for (i, val) in array.iter().enumerate() {
			if map.contains_key(&-val) {
				if let Some(j) = map.get(&-val) {
					let a = (*j as i32) + 1 ;
					let b = (i as i32) + 1;
					result.push(Some((a, b)));
					continue 'outer;
				}
			}
			map.insert(val, i);
		}
		result.push(None);
	}
	return result;
}