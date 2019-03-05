extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<Vec<i32>> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	let input_vec = parser::list_of_things(filename).unwrap();
	return input_vec;
}

fn main() {
	let input_vec = parse_input();
	let ans = algorithmic_heights::sum::threesum(&input_vec[1..]);
	for line in ans {
		match line {
			None => println!("{}", -1),
			Some((a, b, c)) => println!("{} {} {}", a, b, c),
		}
	}
}