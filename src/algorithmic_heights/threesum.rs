use std::collections::HashMap;

pub fn threesum(arrays: &[Vec<i32>]) -> Vec<Option<(i32, i32, i32)>> {
	let mut result = vec![];

	'outer: for array in arrays {

		let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
		for (i, val) in array.iter().enumerate() {
			let list = map.entry(-(*val)).or_insert(vec![]);
			list.push(i);
		}

		for i in 0..(array.len() - 1) {
			if let Some((val, tail)) = array[i..].split_first() {
				for (z, val2) in tail.iter().enumerate() {
					let val3 = val + val2;
					if let Some(list) = map.get(&val3) {
						for k in list {
							let j = i + z + 1;
							if *k > j {
								let a = i as i32 + 1;
								let b = j as i32 + 1;
								let c = *k as i32 + 1;
								result.push(Some((a, b, c)));
								continue 'outer;
							}
						}
					}
				}
			}
		}

		result.push(None);
	}
	return result;
}