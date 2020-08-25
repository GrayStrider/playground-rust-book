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
		fn times(a: u32, b: u32) -> u32 {
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

#[cfg(test)]
mod composition {
	
	trait Ops <T> {
		fn double(&self) -> T;
	}
	
	impl Ops <isize> for isize {
		fn double(&self) -> isize {
			self * 2
		}
	}
	
	#[test]
	fn adds() {
	    assert_eq!(23.double().double(), 92)
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


pub trait OptionMutExt<T> {
	/// Replace the existing `Some` value with a new one.
	///
	/// Returns the previous value if it was present, or `None` if no replacement was made.
	fn replace(&mut self, val: T) -> Option<T>;
	
	/// Replace the existing `Some` value with the result of given closure.
	///
	/// Returns the previous value if it was present, or `None` if no replacement was made.
	fn replace_with<F: FnOnce() -> T>(&mut self, f: F) -> Option<T>;
}

impl<T> OptionMutExt<T> for Option<T> {
	fn replace(&mut self, val: T) -> Option<T> {
		self.replace_with(move || val)
	}
	
	fn replace_with<F: FnOnce() -> T>(&mut self, f: F) -> Option<T> {
		if self.is_some() {
			let result = self.take();
			*self = Some(f());
			result
		} else {
			None
		}
	}
}

