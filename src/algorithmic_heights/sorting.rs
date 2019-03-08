use rand::Rng;

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

fn merge_to_tmp<T: Copy + Ord>(array_a: &[T], array_b: &[T], tmp: &mut [T]) -> usize {
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;

	let total_len = array_a.len() + array_b.len();

	let mut count = 0;

	while k < total_len {
		if j >= array_b.len() {
			tmp[k] = array_a[i];
			i += 1;
		} else if i >= array_a.len() {
			tmp[k] = array_b[j];
			j += 1;
		} else if array_a[i] <= array_b[j] {
			tmp[k] = array_a[i];
			i += 1;
		} else {
			tmp[k] = array_b[j];
			j += 1;
			count += array_a.len() - i;
		}
		k += 1;
	}

	return count;
}

pub fn mer<T: Copy + Ord>(array_a: &[T], array_b: &[T]) -> Vec<T> {
	let mut result = vec![array_a[0]; array_a.len() + array_b.len()];
	merge_to_tmp(array_a, array_b, &mut result);
	return result;
}

pub fn ms<T: Copy + Ord>(array: &mut [T]) -> usize {
	let mut tmp = array.to_vec();
	let mut count = 0;

	let n = array.len();
	let mut sort_size = 1;

	while sort_size < n {
		let next_size = sort_size * 2;

		let mut i = 0;
		for _ in 0..(n/next_size) {
			let j = i + sort_size;
			let k = j + sort_size;
			count += merge_to_tmp(&array[i..j], &array[j..k], &mut tmp[i..k]);
			i += next_size;
		}

		if i + sort_size < array.len() {
			let j = i + sort_size;
			let k = array.len();
			count += merge_to_tmp(&array[i..j], &array[j..k], &mut tmp[i..k]);
		}

		sort_size = next_size;
		array.copy_from_slice(&tmp);
	}

	return count;
}

pub fn inv<T: Copy + Ord>(array: &[T]) -> usize {
	let mut array_copy = array.to_vec();
	return ms(&mut array_copy);
}

pub fn par<T: Copy + Ord>(array: &mut [T]) -> usize {
	// pivot needs to be the first element
	let pivot = array[0];

	let mut i = 0;
	for j in 1..array.len() {
		if array[j] < pivot {
			i += 1;
			array.swap(i, j);
		}
	}

	array.swap(i, 0);

	return i;
}

pub fn par3<T: Copy + Ord>(array: &mut [T]) -> (usize, usize) {
	// pivot needs to be the first element
	let n = array.len() - 1;
	let pivot = array[0];
	array.swap(0, n);

	let mut i = 0;
	let mut k = 0;
	let mut p = n;

	while i < p {
		if array[i] < pivot {
			array.swap(i, k);
			i += 1;
			k += 1;
		} else if array[i] == array[n] {
			p -= 1;
			array.swap(i, p);
		} else {
			i += 1;
		}
	}

	let m = std::cmp::min(p - k, n - p + 1);

	for (a, b) in (k..k + m).zip(n - m + 1..n + 1) {
		array.swap(a, b);
	}

	return (k, n - p + k)
}

pub fn med<T: Copy + Ord>(array: &mut [T], idx: usize) -> T {
	
	let pos = idx - 1;
	let mut a = 0;
	let mut z = array.len();

	let mut rng = rand::thread_rng();

	loop {
		let slice = &mut array[a..z];

		let pivot_idx = rng.gen_range(0, slice.len());

		let n = slice.len() - 1;
		let pivot = slice[pivot_idx];
		slice.swap(pivot_idx, n);

		let mut i = 0;
		let mut k = 0;
		let mut p = n;

		while i < p {
			if slice[i] < pivot {
				slice.swap(i, k);
				i += 1;
				k += 1;
			} else if slice[i] == pivot {
				p -= 1;
				slice.swap(i, p);
			} else {
				i += 1;
			}
		}

		let m = std::cmp::min(p - k, n - p + 1);

		for (a, b) in (k..k + m).zip(n - m + 1..n + 1) {
			slice.swap(a, b);
		}

		if pos > a + (n - p + k) {
			a += k + m;
			z = a + slice.len() - (k + m);
		} else if pos < a + k {
			z = a + k;
		} else {
			// elements in this partition are all equal to pivot
			return pivot;
		}
	}

}

fn three_way_par<T: Copy + Ord>(array: &mut [T]) -> (usize, usize) {
	let mut rng = rand::thread_rng();
	let pivot_idx = rng.gen_range(0, array.len());

	let n = array.len() - 1;
	let pivot = array[pivot_idx];
	array.swap(pivot_idx, n);

	let mut i = 0;
	let mut k = 0;
	let mut p = n;

	while i < p {
		if array[i] < pivot {
			array.swap(i, k);
			i += 1;
			k += 1;
		} else if array[i] == pivot {
			p -= 1;
			array.swap(i, p);
		} else {
			i += 1;
		}
	}

	let m = std::cmp::min(p - k, n - p + 1);

	for (a, b) in (k..k + m).zip(n - m + 1..n + 1) {
		array.swap(a, b);
	}

	return (k, n - p + k)
}

pub fn qs<T: Copy + Ord>(array: &mut [T]) {
	let mut queue = vec![];
	queue.push((0, array.len()));

	while !queue.is_empty() {

		// safe to unwrap because of while loop check
		let (a, z) = queue.pop().unwrap();

		// start and end central partition (inclusive range - slice[s..e+1])
		// elems in central partition are equal to the pivot
		let (slice_pstart, slice_pend) = three_way_par(&mut array[a..z]);

		let pstart = a + slice_pstart;
		let pend = a + slice_pend + 1;

		// if size of left part is MORE THAN 1
		if pstart - a > 1 {
			queue.push((a, pstart));
		}

		//  if size of right part is MORE THAN 1
		if z - pend > 1 {
			queue.push((pend, z))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ins() {
		let mut array = vec![6, 10, 4, 5, 1, 2];
		assert_eq!(ins(&mut array), 12);
	}

	#[test]
	fn test_mer() {
		let a = vec![2, 4, 10, 18];
		let b = vec![-5, 11, 12];
		assert_eq!(mer(&a, &b), [-5, 2, 4, 10, 11, 12, 18]);
	}

	#[test]
	fn test_ms() {
		let mut array = vec![20, 19, 35, -18, 17, -20, 20, 1, 4, 4];
		ms(&mut array);
		assert_eq!(array, [-20, -18, 1, 4, 4, 17, 19, 20, 20, 35]);
	}

	#[test]
	fn test_inv() {
		let mut array = vec![-6, 1, 15, 8, 10];
		assert_eq!(inv(&mut array), 2);
	}

	#[test]
	fn test_par() {
		let mut array = vec![7, 2, 5, 6, 1, 3, 9, 4, 8];
		let pivot = array[0];
		par(&mut array);
		
		for i in 0..6 {
			assert!(array[i] < pivot);
		}
		for i in 7..array.len() {
			assert!(array[i] > pivot);
		}
	}

	#[test]
	fn test_par3() {
		let mut array = vec![4, 5, 6, 4, 1, 2, 5, 7, 4];
		let pivot = array[0];
		par3(&mut array);
		
		for i in 0..2 {
			assert!(array[i] < pivot);
		}

		for i in 2..5 {
			assert_eq!(array[i], pivot);
		}

		for i in 5..array.len() {
			assert!(array[i] > pivot);
		}
	}

	#[test]
	fn test_med() {
		let mut array = vec![2, 36, 5, 21, 8, 13, 11, 20, 5, 4, 1];
		assert_eq!(med(&mut array, 8), 13);
	}

	#[test]
	fn test_qs() {
		let mut array = vec![5, -2, 4, 7, 8, -10, 11];
		qs(&mut array);
		assert_eq!(array, [-10, -2, 4, 5, 7, 8, 11]);
	}
}