fn err() {
	let _x = 5;
	// _x = 6; // error!
}

fn ok() {
	let mut x = 5;
	x = 6; // no problem!
	
	// it’s not so much that the value at _x is changing,
	// but that the binding changed from one i32 to another
	
	// in pattern:
	let (mut _x, _y) = (5, 6);
}

fn interior() {
	use std::sync::Arc;
	
	let x = Arc::new(5);
	let _y = x.clone();
	
	// the mutation is entirely contained inside the structure itself
}

fn exterior() {
	use std::cell::RefCell;
	
	let x = RefCell::new(42);
	
	let y = x.borrow_mut();
	
	let z = x.borrow_mut(); // will panic at runtime
}

// Mutability is a property of either a borrow (&mut) or a binding (let mut)


fn emulate_field_mutability() {
	use std::cell::Cell;
	
	struct Point {
		x: i32,
		y: Cell<i32>, // a mutable memory location
	}
	
	let point = Point { x: 5, y: Cell::new(6) };
	
	point.y.set(7);
	
	println!("y: {:?}", point.y);
}


fn main() {
	// exterior()
}
