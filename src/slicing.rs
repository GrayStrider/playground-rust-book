// slicing
fn main() {
	let a = [0, 1, 2, 3, 4];
	let _complete = &a[..]; // A slice containing all of the elements in a
	let _middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3
	
	let s = String::from("hello, world");
	let first = first_w(&s);
	let first3 = &s[..5];
	
	assert_eq!(first, first3);
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	
	s.len()
}

fn first_w(s: &String) -> &str {
	s.split(", ").collect::<Vec<_>>()[0]
}
