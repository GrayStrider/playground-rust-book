// loops
#[allow(unreachable_code)]
fn loop_() {
	// loop {
	// 	println!("only once");
	// 	break;
	// 	continue; // standart
	// }
	
	fn returns() {
		let mut counter = 0;
		
		let result = loop {
			counter += 1;
			
			if counter == 10 {
				break counter * 2;
			}
		};
		
		println!("The result is {}", result);
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
	
	// reverse
	for i in (0..3).rev() {
		println!("{}", i)
	}
	
	fn iterate() {
		let a = [10, 20, 30, 40, 50];
		
		for element in a.iter() {
			println!("the value is: {}", element);
		}
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
