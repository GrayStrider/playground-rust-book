#[cfg(test)]
mod tests {
	#[test]
	// list comprehension of sorts
	fn iterators() {
		let mapper = |v| ((v * 1235) + 2) / (4 * 16);
		
		let reducer = |mut acc: Vec<i32>, curr: i32| {
			acc.push(mapper(curr));
			acc
		};
		
		type V = Vec<i32>;
		
		let range = || 0..5;
		
		let exp: V = vec![0, 19, 38, 57, 77];
		
		let res3: V = {
			let mut vec: Vec<i32> = vec![];
			for i in range() {
				vec.push(mapper(i));
			}
			vec
		};
		let res: V = range().fold(vec![], reducer);
		
		let res2: V = range().map(mapper).collect();
		
		
		for case in &[res, res2, res3] {
			assert_eq!(case, &exp)
		}
	}
}
