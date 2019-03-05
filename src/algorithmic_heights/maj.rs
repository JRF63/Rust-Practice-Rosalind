use std::hash::Hash;
use std::collections::HashMap;

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