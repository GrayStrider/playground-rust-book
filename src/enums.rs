fn main() {
	messages::main();
	ip::main()
}

mod ip {
	pub fn main() {
		let home = IpAddr {
			kind: IpAddrKind::V4,
			address: String::from("127.0.0.1"),
		};
		
		let loopback = IpAddr {
			kind: IpAddrKind::V6,
			address: String::from("::1"),
		};
	}
	
	enum IpAddrKind {
		V4,
		V6,
	}
	
	fn route(ip_kind: IpAddrKind) {}
	
	fn values() {
		let four = IpAddrKind::V4;
		let six = IpAddrKind::V6;
	}
	
	struct IpAddr {
		kind: IpAddrKind,
		address: String,
	}


	// We can represent the same concept in a more concise way
	// using just an enum, rather than an enum inside a struct,
	// by putting data directly into each enum variant
	
	enum IpAddr2 {
		V4(u8, u8, u8, u8),
		V6(String),
	}
	
	fn consice() {
		let home = IpAddr2::V4(127, 0, 0, 1);
		let loopback = IpAddr2::V6(String::from("::1"));
	}
	
}

mod messages {
	pub fn main() {
		let x: Message = Message::Move { x: 3, y: 4 };
		let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
	}
	
	#[derive(Debug)]
	enum Message {
		Quit,
		ChangeColor(i32, i32, i32),
		Move { x: i32, y: i32 },
		Write(String),
	}
	
	enum BoardGameTurn {
		Move { squares: i32 },
		Pass,
	}
	
	fn constructor() {
		let m = Message::Write("Hello, world".to_string());
		// is the same as
		
		fn make_message(x: String) -> Message {
			Message::Write(x)
		}
		let x = make_message("Hello, world".to_string());
	}
	
	fn map_to() {
		// convert a vector of Strings into a vector of Message::Writes:
		
		let v = vec![
			"Hello".to_string(),
			"World".to_string()
		];
		
		let v1: Vec<Message> = v
			.into_iter()
			.map(Message::Write)
			.collect();
		
		let val = &v1[0];
		println!("{:#?}", v1);
		// [
		//     Write("Hello"),
		//     Write("World"),
		// ]
	}
}

