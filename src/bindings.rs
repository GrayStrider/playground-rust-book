fn patterns() {
	let (x, y) = (1, 2);
}

fn annotations() {
	let x: i8 = 0;
}

fn mutability() {
	let immutable_default = 3;
	let mut mutable = 3;
}

fn shadowing() {
	let x = 2;
	{
		let x = 3; // 3
	}
	
	// 2
}
