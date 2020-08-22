fn main() {
	foo("hello");
	let mut str = "lol";
	let num = 10;
	
	foo(str);
	
	str = str.trim();
	
	bar(str);
	
	increment(num);
	
	println!("{}", num);
}

// uses ref
fn foo(string: &str) {
	println!("{}", string);
}

// mutates the input using ref
fn bar(mut string: &str) -> &str {
	string = string.trim();
	string
}

// returns a new string, does not mutate
fn baz(string: &str) -> &str {
	string.trim()
}

fn increment(mut num: isize) -> isize {
	num = num + 1;
	num
}

