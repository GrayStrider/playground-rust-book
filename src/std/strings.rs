fn main() {
	
	// initialization
	let data = "initial contents";
	let s = data.to_string();
	let s = "initial contents".to_string();
	let s = String::from("initial contents");
	
	// push
	let mut s = String::new();
	s.push_str("bar");
	s.push('!');
	
	// concat
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
	
	
	// format
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");
	let s = format!("{}-{}-{}", s1, s2, s3);
	
	// indexing
	let hello = "Здравствуйте";
	let s = &hello[0..4];
	// You should use ranges to create string slices with caution,
	// because doing so can crash your program ("not a char boundary").
	
	// iteration
	let word = "नमस्ते";
	word.chars();
	word.bytes();
}
