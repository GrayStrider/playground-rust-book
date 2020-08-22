// list of items, such as the lines of text in
// a file or the prices of items in a shopping cart.

use std::ops::Add;

fn main() {
	let v: Vec<i32> = Vec::new();
	let mut v = vec![1, 2, 3];
	let a = [1, 2, 3];
	
	// update
	v.push(4);
	v.append(vec![5, 6].as_mut());
	
	// access, typesafe
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}
	match a.get(2) {
		Some(third) => println!("Found: {:?}", third),
		None => println!("None")
	}
	
	// by index, can crash
	&v[2];
	&a[2];
	
	// iterate
	for i in &v {
		println!("i: {}", i);
	}
	
	let c: Vec<i32> = v.iter()
		.map(|i| i.add(1))
		.collect();
	
	println!("incremented v: {:?}", c);
}
