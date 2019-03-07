extern crate rosalind;
use rosalind::{algorithmic_heights, utility::parser};

fn parse_input() -> Vec<Vec<usize>> {
	let filename = format!("input_dataset/{}", &parser::cmdline_arguments()[1]);
	return parser::list_of_things(filename).unwrap();
}

fn main() {
	let input_arr = parse_input();
	let num_nodes = input_arr[0][0];
	let digraph = algorithmic_heights::graph::create_digraph(&input_arr[1..]);
	let ans = algorithmic_heights::graph::bfs(&digraph, num_nodes);
	let res = parser::vec_to_string(ans);
	println!("{}", res);
}