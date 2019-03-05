extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<i32> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	return parser::list_of_things(filename).unwrap().pop().unwrap();
}

fn main() {
	let mut input_arr = parse_input();
	let res = algorithmic_heights::sorting::ins(&mut input_arr);
	println!("{}", res);
}