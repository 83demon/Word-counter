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

#[cfg(test)]
mod test {

	use super::*;

	#[test]
  	fn test_char_multi_line_count() {
		let source = "ac s e e\x0Bc f\nr t\tr i\x0As s\rw";
       	let expected_chars_counter = 26;
 		let actual_chars_counter = count_characters(& source);
        assert_eq!(expected_chars_counter, actual_chars_counter);
	}

	// \x0B stands for \v, \x0A stands for \f
	#[test]
  	fn test_words_multi_line_count() {
		let source = "ac s e e\x0Bc f\nr t\tr i\x0As s\rw";
       	let expected_words_counter = 13;
 		let actual_words_counter = count_words(& source);
        assert_eq!(expected_words_counter, actual_words_counter);
	}

	#[test]
	fn test_lines_count() {
		let source = "ac s e e\x0Bc f\nr t\tr i\x0As s\rw";
		let expected_lines_count = 3;
		let actual_lines_count = count_lines(& source);
		assert_eq!(expected_lines_count, actual_lines_count);
	}
}
