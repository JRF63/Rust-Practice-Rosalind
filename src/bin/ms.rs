extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<i32> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let mut input_vec = parser::list_of_things(filename).unwrap();
	let nums = input_vec.pop().unwrap();
	return nums;
}

fn main() {
	let mut nums = parse_input();

	algorithmic_heights::ms(&mut nums);
	let res = parser::vec_to_string(nums);
	println!("{}", res);
}