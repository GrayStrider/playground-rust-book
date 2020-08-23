fn main() {
	// patterns
	let (x, y) = (1, 2);
	
	// type annotations
	let x: i8 = 0;
	
	// mutability
	let immutable_default = 3;
	#[allow(unused_mut)]
		let mut mutable = 3;
	
	// shadowing, scope
	let x = 2;
	{
		let x = 3; // 3
	}
	x; // 2
}


fn d() {
	fn comments() {
		
		/// Adds one to the number given.
		///
		/// # Examples
		///
		/// ```
		/// let five = 5;
		///
		/// assert_eq!(6, add_one(5));
		/// # fn add_one(x: i32) -> i32 {
		/// #     x + 1
		/// # }
		/// ```
		fn add_one(x: i32) -> i32 {
			x + 1
		}

// / You can use the rustdoc tool to generate
// / HTML documentation from these doc comments,
// / and also to run the code examples as tests!
	}
	
}

