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

pub fn par3<T: Copy + Ord>(array: &mut [T]) -> usize {
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

	return k;
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
			return slice[k];
		}
	}

}