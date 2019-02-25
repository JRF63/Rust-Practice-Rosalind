use std::cmp::{Eq, Ord};
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

pub fn deg<T: Eq + Hash + Ord + Copy>(input_arr: &[Vec<T>]) -> Vec<usize> {
	let mut graph: HashMap<T, HashSet<T>> = HashMap::with_capacity(input_arr.len());
	for edge in input_arr {
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

	let mut sorted_idx = vec![];
	for (key, _) in &graph {
		sorted_idx.push(*key);
	}
	sorted_idx.sort();

	let result = sorted_idx
					.into_iter()
					.map(|k| graph[&k].len())
					.collect();
	return result;
}