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