extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<Vec<i32>> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	return parser::list_of_things(filename).unwrap();
}

fn main() {
	let input_arr = parse_input();
	let ans = algorithmic_heights::maj(&input_arr[1..], -1);
	let res = parser::vec_to_string(ans);
	println!("{}", res);
}