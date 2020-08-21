fn IF() {
	let x = 5;
	
	if x == 5 {
		println!("x is five!");
	} else if x == 6 {
		println!("x is six!");
	} else {
		println!("x is not five or six :(");
	}
}

fn if_expression() {
	let x = 5;
	
	let y = if x == 5 {
		10
	} else {
		15
	}; // y: i32
	
	// Which we can (and probably should) write like this:
	
	let x = 5;
	
	let y = if x == 5 { 10 } else { 15 }; // y: i32
	
	// This works because if is an expression.
	// The value of the expression is the value of the last
	// expression in whichever branch was chosen.
	// An if without an else always results in () as the value.
}


/// loops
fn loop_() {
	loop {
		println!("only once");
		break;
		continue; // standart
	}
}


fn while_() {
	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool
	
	while !done {
		x += x - 3;
		
		println!("{}", x);
		
		if x % 5 == 0 {
			done = true;
		}
	}
}


fn for_() {
	for x in 0..10 {
		println!("{}", x); // x: i32
	}
	
	fn enumerate() {
		// on ranges:
		for (i, j) in (5..10).enumerate() {
			println!("i = {} and j = {}", i, j);
		}
		
		// on iterators:
		let lines = "hello\nworld".lines();
		
		for (linenumber, line) in lines.enumerate() {
			println!("{}: {}", linenumber, line);
		}
	}
}


fn loop_labels() {
	'outer: for x in 0..10 {
		'inner: for y in 0..10 {
			if x % 2 == 0 { continue 'outer; } // continues the loop over x
			if y % 2 == 0 { continue 'inner; } // continues the loop over y
			println!("x: {}, y: {}", x, y);
		}
	}
}
