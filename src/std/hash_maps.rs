#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;


fn main() {
	let mut scores = HashMap::new();
	
	let blue = String::from("Blue");
	let yellow = String::from("Yellow");
	
	scores.insert(&blue, 10);
	scores.insert(&yellow, 50);
	
	// overwriting
	scores.insert(&blue, 23);
	
	// modify if exists, otherwise insert new
	scores
		.entry(&blue)
		.and_modify(|x| *x += 1)
		.or_insert(27);
	
	println!("{:?}", scores);
	// teams()
	let map = word_count(
		String::from("one one Two One three   tht tht Tht  HT"));
	println!("{:#?}", map);
}

fn teams() {
	let teams = vec![
		String::from("Blue"),
		String::from("Yellow")
	];
	let initial_scores = vec![10, 50];
	
	let scores: HashMap<_, _> =
		teams.into_iter()
		     .zip(initial_scores.into_iter())
		     .collect();
	
	println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
	
	// access
	
	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
	
	// iteration
	for (team, value) in &scores { /*...*/ };
	scores.iter().for_each(|(team, value)| { /*...*/ });
}

fn ownership() {
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");
	
	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	// field_name and field_value are invalid at this point!
}

fn word_count(text: String) -> HashMap<String, u32> {
	text
		.split_whitespace()
		.map(|word| word.to_lowercase())
		.fold(HashMap::new(), |mut map, word| {
			*map.entry(word).or_insert(0) += 1;
			map
		})
}

// unnecessary variables, word_count is more concise
fn word_count_via_reference(text: String) -> HashMap<String, u32> {
	let mut map = HashMap::new();
	let words = text
		.split_whitespace()
		.map(|w| w.to_lowercase());
	for word in words {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	map
}
