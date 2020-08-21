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

fn main() {
	let x: Message = Message::Move { x: 3, y: 4 };
	
	let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
}

fn constructor() {
	let m = Message::Write("Hello, world".to_string());
	// is the same as
	
	fn foo(x: String) -> Message {
		Message::Write(x)
	}
	
	let x = foo("Hello, world".to_string());
}

fn convert() {
	// convert a vector of Strings into a vector of Message::Writes:
	
	let v = vec![
		"Hello".to_string(),
		"World".to_string()
	];
	
	let v1: Vec<Message> = v.into_iter()
		.map(Message::Write)
		.collect();
	
	let val = &v1[0];
}
