extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> u32 {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	return parser::list_of_things(filename).unwrap()[0][0];
}

fn main() {
	let n = parse_input();
	let result = algorithmic_heights::fibo(n);
	println!("{}", result);
}