#[allow(unused_assignments)]

fn primitives() {
	// bool
	let x = true;
	let y: bool = false;
	
	// arrays
	// If you’re unsure whether to use an array or
	// a vector, you should probably use a vector
	let a = [1, 2, 3];
	let a:[i8; 2] = [3, 2];
	a[1];
	
	let a = ["foo"; 10]; // 10 foos
	
	// tuples
	let x = (1, "hello");
	let x: (i32, &str) = (1, "hello");
	
	let mut x = (1, 2); // x: (i32, i32)
	let y = (2, 3); // y: (i32, i32)
	
	x = y;
	
	println!("{:#?}", x); // (2, 3)
	

	
}


fn main() {
	primitives()
}
