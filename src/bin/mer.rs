extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> (Vec<i32>, Vec<i32>) {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let mut input_vec = parser::list_of_things(filename).unwrap();
	let array_b = input_vec.pop().unwrap();
	input_vec.pop();
	let array_a = input_vec.pop().unwrap();
	return (array_a, array_b);
}

fn main() {
	let (array_a, array_b) = parse_input();
	let ans = algorithmic_heights::mer(&array_a[..], &array_b[..]);
	let res = parser::vec_to_string(ans);
	println!("{}", res);
}