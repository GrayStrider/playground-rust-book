use std::ops::RangeTo;

mod user;
mod rectangle;

fn main() {
	let point = Point { x: 3, y: 2 };
	point.y;
	
	let mut mutable_p = Point { x: 2, y: 3 };
	mutable_p.x += 2;
}


// By convention, structs begin with a capital letter and are camel cased
struct Point {
	x: i32,
	y: i32,
}

// store references, lifetimes required
struct Foo<'a> {
	x: &'a i32
}

impl<'a> Foo<'a> {
	fn method(&self) -> &'a i32 { self.x }
}

fn mutable_pointers() {
	struct PointRef<'a> {
		x: &'a mut i32,
		y: &'a mut i32,
	}
	
	let mut point = Point { x: 0, y: 0 };
	
	{
		let r = PointRef { x: &mut point.x, y: &mut point.y };
		
		*r.x = 5;
		*r.y = 6;
	}
	
	assert_eq!(5, point.x);
	assert_eq!(6, point.y);
}

fn spread_operator() {
	struct Point3d {
		xy: Point,
		z: i32,
	}
	
	let point = Point { x: 3, y: 4 };
	let point3d = Point3d { xy: point, z: 0 };
	
	// rust arrays have a fixed length! use vector or convert into iterable
	let origin = [1, 2];
	let new: (RangeTo<[i32; 2]>, i32) = (..origin, 3);
}

fn tuple_struct() {
	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);
	
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
	
	// Good names are important, and while values in a tuple struct
	// can be referenced with dot notation as well, a struct gives us
	// actual names, rather than positions.
	//
	// There is one case when a tuple struct is very useful, though,
	// and that is when it has only one element. We call this the
	// ‘newtype’ pattern, because it allows you to create a new type
	// that is distinct from its contained value and also
	// expresses its own semantic meaning:
	
	struct Inches(i32);
	
	let length = Inches(10);
	
	let Inches(integer_length) = length;
	println!("length is {} inches", integer_length);
}

fn no_members() {
	struct Electron;
	
	let x = Electron;
	
	// Such a struct is called ‘unit-like’ because it resembles
	// the empty tuple, (), sometimes called ‘unit’. Like a
	// tuple struct, it defines a new type.
	//
	// This is rarely useful on its own (although sometimes it
	// can serve as a marker type), but in combination with
	// other features, it can become useful.
}
