use std::io;

fn count_characters(string_holder: & str) -> usize {
	string_holder.len()
}

fn count_words(string_holder: & str) -> usize {
	string_holder.split_whitespace().collect::<Vec<_>>().len()
}

fn count_lines(string_holder: & str) -> usize {
	string_holder.split('\n').collect::<Vec<_>>().len()
}


fn main() {
    println!("Hello, world!");

	let mut string_holder = String::new();

	io::stdin()
		.read_line(& mut string_holder)
		.expect("A string is expected here");

	let chars_len = count_characters(& string_holder);
	let words_count = count_words(& string_holder);
	let lines_count = count_lines(& string_holder);

	println!("You entered {string_holder} with a len of {chars_len} characters and words counter of {words_count}. Number of lines are {lines_count}.");
}
