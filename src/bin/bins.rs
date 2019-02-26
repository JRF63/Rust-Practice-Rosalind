extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> (Vec<i32>, Vec<i32>) {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let mut input_vec = parser::list_of_things(filename).unwrap();
	let nums = input_vec.pop().unwrap();
	let sorted_arr = input_vec.pop().unwrap();
	return (sorted_arr, nums);
}

fn main() {
	let (sorted_arr, nums) = parse_input();

	let res = nums.into_iter()
				  .map(|n| algorithmic_heights::bins(&sorted_arr, n).to_string())
				  .collect::<Vec<String>>()
				  .join(" ");

	println!("{}", res);
}