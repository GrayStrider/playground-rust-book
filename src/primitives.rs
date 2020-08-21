#[allow(unused_assignments)]

fn primitives() {
	let _x = true;
	let _y: bool = false;
	
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
	let _a = [1, 2, 3];
	let a:[i8; 2] = [3, 2];
	a[1];
	
	// slicing
	let a = [0, 1, 2, 3, 4];
	let _complete = &a[..]; // A slice containing all of the elements in a
	let _middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3
	
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
