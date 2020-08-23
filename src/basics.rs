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
