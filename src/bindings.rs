fn patterns() {
	let (_x, _y) = (1, 2);
}

fn annotations() {
	let _x: i8 = 0;
}

fn mutability() {
	let _immutable_default = 3;
	let mut _mutable = 3;
}

fn shadowing() {
	let _x = 2;
	{
		let _x = 3; // 3
	}
	
	// 2
}
