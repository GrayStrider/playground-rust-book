fn main() {
	let _user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	
	user1.email = String::from("anotheremail@example.com");
	
	let _user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};
}


struct User {
	username: String,
	email: String,
	// String: we want instances of this struct to own
	// all of its data and for that data to be valid
	// for as long as the entire struct is valid
	sign_in_count: u64,
	active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);


fn color() {
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}


