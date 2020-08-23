use std::iter::repeat;

// default impl
trait Methodize: Sized {
	fn methodize<F, R>(self, f: F) -> R
		where F: FnOnce(Self) -> R
	{
		f(self)
	}
}
impl<T> Methodize for T {}

fn main() {
	println!("{:?}", (1).methodize(repeat));
	println!("{:?}", (1).methodize(|x| 9 - x));
}
