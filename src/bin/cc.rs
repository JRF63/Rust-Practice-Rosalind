extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<Vec<usize>> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	return parser::list_of_things(filename).unwrap();
}

fn main() {
	let input_arr = parse_input();
	let num_nodes = input_arr[0][0];
	let graph = algorithmic_heights::graph::create_graph(&input_arr[1..]);
	let res = algorithmic_heights::graph::cc(&graph, num_nodes);
	println!("{}", res);
}