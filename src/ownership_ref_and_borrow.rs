fn hand_back() {
	fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
		// do stuff with v1 and v2
		
		// hand back ownership, and the result of our function
		(v1, v2, 42)
	}
	
	let v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];
	
	let (v1, v2, answer) = foo(v1, v2);
	
	// can't use v1 / v2 here!
}

fn using_references() {
	fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
		// do stuff with v1 and v2
		
		// return the answer
		42
	}
	
	let v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];
	
	let answer = foo(&v1, &v2);

// we can use v1 and v2 here!
}

fn mut_references() {
	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}
	println!("{}", x);
	// x is 6!
}

fn scopes() {
	let mut x = 5;
	let y = &mut x;
	
	*y += 1;
	
	println!("{}", x);
	// cannot borrow `x` as immutable because it is also borrowed as mutable
	// The scopes conflict: we can’t make an &x while y is in scope
	
	//that's mut_references requires additional scope inside
}

fn issues() {
	let mut v = vec![1, 2, 3];
	
	for i in &v {
		println!("{}", i);
		v.push(34);
		// We can’t modify v because it’s borrowed by the loop
	}
}
