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

	let ans = algorithmic_heights::bins(&sorted_arr, &nums);
	let res = parser::vec_to_string(ans);
	println!("{}", res);
}