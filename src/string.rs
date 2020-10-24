pub mod validate;

#[allow(dead_code)]
pub fn reverse_word(string: &str) -> String {
	string.chars().rev().collect()
}

#[allow(dead_code)]
pub fn is_panagram(string: &str) -> bool {
	let mut letters: [bool; 26] = [false; 26];

	for c in string.to_lowercase().chars() {
		if c.is_ascii_alphabetic() {
			let index = (c as usize) - 97;
			letters[index] = true;
		}
	}

	for c in &letters {
		if !c {
			return false;
		}
	}

	true
}

#[allow(dead_code)]
pub fn missing_char_for_panagram(string: &str) -> String {
	let mut letters: [bool; 26] = [false; 26];

	for c in string.to_lowercase().chars() {
		if c.is_ascii_alphabetic() {
			let index = (c as usize) - 97;
			letters[index] = true;
		}
	}

	let mut missing_chars = String::new();

	for (i, v) in letters.iter().enumerate() {
		if !v {
			missing_chars.push((i as u8 + 97) as char);
		}
	}

	missing_chars
}
