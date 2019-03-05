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

pub fn inv<T: Copy + Ord>(input: &[T]) -> usize {
	let mut array = input.to_vec();
	return ms(&mut array);
}