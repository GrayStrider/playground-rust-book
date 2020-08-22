trait Methodize: Sized {
	fn methodize<F, R>(self, f: F) -> R
		where F: FnOnce(Self) -> R
	{
		f(self)
	}
}
impl<T> Methodize for T {}

fn main() {
	println!("{:?}", (1).methodize(std::iter::repeat));
	println!("{:?}", (1).methodize(|x| 9 - x));
}
