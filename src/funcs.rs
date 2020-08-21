fn print_number(x: i32) {
	println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
	println!("sum is: {}", x + y);
	// returns () !
}

fn add_one(x: i32) -> i32 {
	x + 1 // no semi
}

/// Rust is primarily an expression-based language.
/// There are only two kinds of statements, and everything else is an expression.
/// So what's the difference? Expressions return a value, and statements do not

fn foo(x: i32) -> i32 {
	return x;
	
	// we never run this code!
	x + 1
}

fn diverges() -> ! {
	panic!("This function never returns!");
}

/// A diverging function can be used as any type:
fn diverging_assignment() {
	let x: i32 = diverges();
	let x: String = diverges();
}

/// function pointers
fn plus_one(i: i32) -> i32 {
	i + 1
}

fn pointers() {
	// without type inference
	let f: fn(i32) -> i32 = plus_one;

// with type inference
	let f = plus_one;
	
	let six = f(5);
}
