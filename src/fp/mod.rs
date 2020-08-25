use std::ops::{Add, Mul};


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
	
	#[test]
	fn pipe_() {
		fn times(a: u32, b: u32) -> u32{
			a * b
		}
		//
		// let add5 = add(5);
		// let sub3 = add(-3);
		// let double = mult(2);
		//
		fn add2(x: i32) -> i32 {
			x + 2
		}
		
		assert_eq!(pipe!(10 => add2), 12)
	}
}

fn main() {
	let adder = |x: i32| move |y: i32| x + y;
}

pub fn add<T: Add>(x: i32) -> impl Fn(i32) -> i32 {
	move |y| x + y
}

pub fn mult<T: Mul + Copy>(x: i32) -> impl Fn(i32) -> i32 {
	move |y| x * y
}

pub fn pipe<T>(init: i32, funcs: &[fn(i32) -> i32]) -> i32 {
	0
}
