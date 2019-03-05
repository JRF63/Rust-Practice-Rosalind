extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<i32> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let mut input_vec = parser::list_of_things(filename).unwrap();
	let output = input_vec.pop().unwrap();
	return output;
}

fn main() {
	let array = parse_input();
	let ans = algorithmic_heights::sorting::inv(&array);
	println!("{}", ans);
}