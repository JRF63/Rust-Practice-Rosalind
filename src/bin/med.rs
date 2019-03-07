extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> (Vec<i32>, usize) {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let mut input_vec = parser::list_of_things(filename).unwrap();
	
	let idx = input_vec.pop().unwrap()[0];
	let array = input_vec.pop().unwrap();
	return (array, idx as usize);
}

fn main() {
	let (mut array, idx) = parse_input();
	let res = algorithmic_heights::sorting::med(&mut array, idx);
	println!("{}", res);
}