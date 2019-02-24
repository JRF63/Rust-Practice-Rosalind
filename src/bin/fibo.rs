extern crate rosalind;
use rosalind::{algorithmic_heights, utility};
use utility::parser;

fn main() {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let v: Vec<Vec<i32>> = parser::list_of_things(filename).unwrap();
	let n = v[0][0];
	let result = algorithmic_heights::fibo(n);
	println!("{}", result);
}