pub fn fibo(n: u32) -> u32 {
	let mut i = 0;
	let mut j = 1;
	for _ in 0..n {
		let tmp = i;
		i = j;
		j = tmp + j;
	}
	return i;
}