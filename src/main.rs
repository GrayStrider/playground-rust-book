fn main() {
	let _x = 5;
	
	let controller: Controller = |data| {
		let data = data.parse()
			.expect("Bad string");
		Res { data, meta: 10 }
	};
}

type Controller = fn (data: str) -> Res;

struct Res {
	data: String,
	meta: i32
}
