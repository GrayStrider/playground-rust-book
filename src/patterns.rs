fn match_against_literals() {
	let x = 1;
	
	match x {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		_ => println!("anything"),
	}
}

fn shadowing() {
	let x = 1;
	let c = 'c';
	
	let x = c;
	println!("x: {} c: {}", x, c);
	
	println!("x: {}", x)
	// x: c c: c
	// x: 1
	
	// the value of x outside the scope of the match
	// has no bearing on the value of x within it.
	// Because we already have a binding named x, this new x shadows it.
}

fn multiple() {
	let x = 1;
	
	match x {
		1 | 2 => println!("one or two"),
		3 => println!("three"),
		_ => println!("anything"),
	}
}

fn destructuring() {
	struct Point {
		x: i32,
		y: i32,
	}
	
	let origin = Point { x: 0, y: 0 };
	
	let Point { x, y } = origin;
	println!("({},{})", x, y);
	
	let Point { x: x1, y: y1 } = origin;
	println!("({},{})", x1, y1);
	
	let Point { x, .. } = origin;
	println!("x is {}", x);
	
	let Point { y, .. } = origin;
	println!("y is {}", y);
	
	// This ‘destructuring’ behavior works on any
	// compound data type, like tuples or enums.
}

fn ignoring() {
	let val = String::new();
	
	// use _ in a pattern to disregard the type and value
	match val.parse::<String>() {
		Ok(value) => println!("string: {}", value),
		Err(_) => println!("failed to parse"),
	}
	
	// Here, the String created will be dropped immediately, as it’s not bound:
	let _ = String::from("  hello  ").trim();
	
	// ignore multiple values
	enum OptionalTuple {
		Value(i32, i32, i32),
		Missing,
	}
	
	let x = OptionalTuple::Value(5, -2, 3);
	
	match x {
		OptionalTuple::Value(..) => println!("Got a tuple!"),
		OptionalTuple::Missing => println!("No such luck."),
	}
}

fn reference() {
	let x = 5;
	
	let r = &x;
	println!("Got a reference to {}", r);
	
	fn mutable() {
		let mut x = 5;
		
		let mr = &mut x;
		println!("Got a mutable reference to {}", mr);
	}
}

fn ranges() {
	let x = 1;
	
	match x {
		n @ 1..=5 => println!("one through five: {}", n),
		_ => println!("anything"),
	}
	
	let x = '💅';
	
	match x {
		'a'..='j' => println!("early letter"),
		'k'..='z' => println!("late letter"),
		_ => println!("something else"),
	}
}

fn bindings() {
	let x = 1;
	
	match x {
		e @ 1..=5 => println!("got a range element {}", e),
		_ => println!("anything"),
	}
	
	fn some_person() {
		#[derive(Debug)]
		struct Person {
			name: Option<String>,
		}
		
		let name = "Steve".to_string();
		let x: Option<Person> = Some(Person { name: Some(name) });
		if let Some(Person { name: ref a @ Some(_), .. }) =
		x { println!("{:?}", a) }
	}
}
