fn implicit() {
	fn foo(x: &i32) {}
}

fn explicit() {
	// The 'a reads ‘the lifetime a’.
	fn bar<'a>(x: &'a i32) {}
}

fn in_structs() {
	struct Foo<'a> {
		x: &'a i32,
	}
	
	let y = &5; // this is the same as `let _y = 5; let y = &_y;`
	let f = Foo { x: y };
	
	println!("{}", f.x);
}

fn impl_blocks() {
	struct Foo<'a> {
		x: &'a i32,
	}
	
	impl<'a> Foo<'a> {
		fn method(&self) -> &'a i32 { self.x }
	}
	
	let y = &5; // this is the same as `let _y = 5; let y = &_y;`
	let f = Foo { x: y };
	
	println!("x is: {}", f.method());
}

static FOO: i32 = 5;

fn static_lifetime() {
	let x: &'static str = "Hello, world.";
	let x: &'static i32 = &FOO;
}
