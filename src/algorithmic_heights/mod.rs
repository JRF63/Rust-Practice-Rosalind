use std::cmp::Ord;
use std::marker::Copy;

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

fn binary_search<T: Ord + Copy>(sorted_arr: &Vec<T>, n: T) -> i32 {
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
			// not found
			return -1;
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

pub fn bins<T: Ord + Copy>(sorted_arr: &Vec<T>, n: T) -> i32 {
	// add one to get 1 based index
	match binary_search(sorted_arr, n) {
		-1 => -1,	// preserve -1
		x  => x + 1,
	}
}