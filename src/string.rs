pub fn reverse_word(string: &str) -> String {
	string.chars().rev().collect()
}

pub fn panagram_check(string: &str) -> bool {
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
