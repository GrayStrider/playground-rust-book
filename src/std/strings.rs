fn main() {
	
	// initialization
	let data = "initial contents";
	let s = data.to_string();
	let s = "initial contents".to_string();
	let s = sf("initial contents");
	// literal
	let two_hearts = '💕'; // immutable! hardcoded in at compile time
	let c: &'static str = "hello"; // exists for the duration of the program
	// concat
	let s1 = sf("Hello, ");
	let s2 = sf("world!");
	let mut s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
	s3.extend(sf("!!!").chars());
	assert_eq!(s3, "Hello, world!!!!");
	s3.retain(|c| c != 'o');
	assert_eq!(s3, "Hell, wrld!!!!");
	// format
	let s = format!("{}-{}-{}", sf("tic"), sf("tac"), sf("toe"));
	
	// mutate
	let mut s = String::new();
	s.push_str("bar");
	s.push('!');
	
	// indexing
	let hello = "Здравствуйте";
	let first_4 = &hello[0..4];
	// You should use ranges to create string slices with caution,
	// because doing so can crash your program ("not a char boundary").
	assert_eq!(first_4, "Зд"); // 2 bytes per letter!
	assert_eq!(hello.get/*_mut*/(0..4), Some("Зд")); // typesafe
	assert_eq!(hello.len(), 24);
	assert_eq!(hello.find("й"), Some(18));
	assert_eq!(hello.find("123"), None);
	
	// iteration
	let word = "नमस्ते";
	word.chars();
	word.bytes();
}

fn sf(s: &str) -> String { String::from(s) }
