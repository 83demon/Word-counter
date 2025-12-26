use std::io;

fn count_characters(string_holder: & str) -> usize {
	string_holder.len()
}

fn count_words(string_holder: & str) -> usize {
	string_holder.split_whitespace().collect::<Vec<_>>().len()
}

fn count_lines(string_holder: & str) -> usize {
	string_holder.lines().count()
}


fn main() {
    println!("Hello, world!");

	let mut string_holder = String::new();

	io::stdin()
		.read_line(& mut string_holder)
		.expect("A string is expected here");

	let string_holder = string_holder.trim();

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
		let source = "ac s e e\x0Bc f\nr t\tr i\x0As s\rw\n";
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

	#[test]
	fn test_words_with_punctuation_boundaries() {
		let source = "Hello,world";
		let expected_words = 1;
		let actual_words = count_words(& source);
		assert_eq!(actual_words, expected_words, "Failed to split words on punctuation");

		let source_2 = "Wait...what? (End)";
		let expected_res_2 = 2;
		assert_eq!(count_words(& source_2), expected_res_2);
	}

	#[test]
	fn test_words_with_complex_hyphens_and_apostrophes() {
		let source = "It's a state-of-the-art solution.";
		let expected_words = 4; 
		assert_eq!(count_words(& source), expected_words);
	}

	#[test]
	fn test_line_edge_cases() {
		assert_eq!(count_lines(""), 0, "Empty string should have 0 lines");
		assert_eq!(count_lines("Line 1\r\nLine 2\r\n"), 2, "Windows CRLF should count as one break");
		assert_eq!(count_lines("Line 1\n"), 1, "Trailing newline should not create a phantom empty line");
	}
}
