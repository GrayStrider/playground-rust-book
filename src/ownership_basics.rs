fn move_semantics() {
	let v = vec![1, 2, 3];
	
	// let v2 = v; // will not compile, use of moved value
	
	println!("v[0] is: {}", v[0]);
}

fn takes_ownership() {
	fn take(v: Vec<i32>) {
		// what happens here isn’t important.
	}
	
	let v = vec![1, 2, 3];
	
	// take(v); // ownership gets taken, and we can't use v anymore
	
	println!("v[0] is: {}", v[0]);
}

fn copy_trait() {
	let v = 1;
	
	let v2 = v;
	
	// all primitive types implement copy trait
	println!("v is: {}", v);
}


fn main() {
	move_semantics()
}
