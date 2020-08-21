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

// loops
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

fn match_() {
	let x = 5;
	
	match x {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		4 => println!("four"),
		5 => println!("five"),
		_ => println!("something else"),
	}
	
	// as expression
	let number = match x {
		1 => "one",
		2 => "two",
		3 => "three",
		4 => "four",
		5 => "five",
		_ => "something else",
	};
	
	fn enums() {
		enum Message {
			Quit,
			ChangeColor(i32, i32, i32),
			Move { x: i32, y: i32 },
			Write(String),
		}
		
		fn quit() { /* ... */ }
		fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
		fn move_cursor(x: i32, y: i32) { /* ... */ }
		
		fn process_message(msg: Message) {
			match msg {
				Message::Quit => quit(),
				Message::ChangeColor(r, g, b) => change_color(r, g, b),
				Message::Move { x: x, y: y } => move_cursor(x, y),
				Message::Write(s) => println!("{}", s),
			};
		}
		
		process_message(Message::Quit);
		// /*Message without variants*/ will fail at compile time
	}
}

fn main() {

}
