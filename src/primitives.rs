#[allow(unused_assignments)]

fn primitives() {
	let _x = true;
	let _y: bool = false;
	
	
	// literal
	let _x = "bar"; // immutable! hardcoded at compile time
	// mutable
	let mut mutable = String::from("hello");
	mutable.push_str(", world");
	
	// char
	let _x = 'x';
	let _two_hearts = '💕';
	
	let _x = 42; // x has type i32
	let _y = 1.0; // y has type f64
	
	
	// fixed
	let _signed: i8; // -8 - 7
	let _unsigned: u8; // 0 - 15
	
	let _variable: isize;
	let _variable: usize;
	
	let _floating: f32; //f64
	
	// arrays
	// If you’re unsure whether to use an array or
	// a vector, you should probably use a vector
	let _a = [1, 2, 3];
	let a:[i8; 2] = [3, 2];
	a[1];
	
	let _a = ["foo"; 10]; // 10 foos
	
	// tuples
	let _x = (1, "hello");
	let _x: (i32, &str) = (1, "hello");
	
	let mut _x = (1, 2); // x: (i32, i32)
	let y = (2, 3); // y: (i32, i32)
	
	_x = y;
	
	
	// destructuring
	let (_x, _y, _z) = (1, 2, 3);
	
	
	// functions
	fn foo(x: i32) -> i32 { x }
	
	let _x: fn(i32) -> i32 = foo;
	
}
