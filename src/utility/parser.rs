use std::{
	env::self,
	fs::File,
	io::{self, Error, ErrorKind, BufReader, BufRead},
	str::FromStr,
	path::Path,
};

pub fn cmdline_arguments() -> Vec<String>
{
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		panic!("enter filename");
	}
	args
}

pub fn list_of_things<T: FromStr>(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<T>>> {
	let file = File::open(filename).expect("error opening file");
	let buf = BufReader::new(file);

	let mut result: Vec<Vec<T>> = vec![];
	for line in buf.lines() {
		let mut tmp: Vec<T> = vec![];
		for word in line.unwrap().split_whitespace() {
			let r = match T::from_str(&word) {
				Ok(x) => x,
				Err(_) => return Err(Error::new(ErrorKind::Other, "error parsing"))
			};
			tmp.push(r);
		}
		result.push(tmp);
	}
	Ok(result)
}