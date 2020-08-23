use std::ops::Add;

fn main() {
	// if let
	let x = Some(0);
	if let Some(i) = x {
		println!("Not None: {:#?}", i.to_string() + "!");
	}
	
	// ternary
	let x = 5;
	let y = if x == 5 { 10 } else { 15 };
	
	// functions
	fn foo(x: i32) -> i32 { x }
	let x: fn(i32) -> i32 = foo;
	// closure (arrow fn, anonimous fn)
	let x = |x: i32| x;
	// currying
	let adder = |x: i32| move |y: i32| y.add(x);
	let plus_one = adder(1);
	let three = plus_one(2);
	// panics, diverges
	fn error() -> ! { panic!("Error; return ()") }
	
	// patterns
	let (x, y) = (1, 2);
	
	// type annotations
	let x: i8 = 0;
	// "!" can be used as any type
	fn diverges() { let any: f32 = error(); }
	
	// mutability
	let immutable_default = 3;
	#[allow(unused_mut)]
		let mut mutable = 3;
	
	// shadowing, scope
	let x = 2;
	{ let x = 3; }
	x; // 2
	
	// destructuring
	let (x, y, z) = (1, 'c', 3.9);
	
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

