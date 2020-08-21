fn primitives() {
	let x = true;
	let y: bool = false;
	
	let x = 'x';
	let two_hearts = '💕';
	
	let x = 42; // x has type i32
	let y = 1.0; // y has type f64
	
	
	/// fixed
	let signed: i8; // -8 - 7
	let unsigned: u8; // 0 - 15
	
	let variable: isize;
	let variable: usize;
	
	let floating: f32; //f64
	
	/// arrays
	let a = [1, 2, 3];
	let a:[i8; 2] = [3, 2];
	a[1];
	
	/// slicing
	let a = [0, 1, 2, 3, 4];
	let complete = &a[..]; // A slice containing all of the elements in a
	let middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3
	
	/// tuples
	let x = (1, "hello");
	let x: (i32, &str) = (1, "hello");
	
	let mut x = (1, 2); // x: (i32, i32)
	let y = (2, 3); // y: (i32, i32)
	
	x = y;
	
	
	/// destructuring
	let (x, y, z) = (1, 2, 3);
	
	/// access
	x.1;
	
	
	/// functions
	fn foo(x: i32) -> i32 { x }
	
	let x: fn(i32) -> i32 = foo;
	
}
