use regex::Regex;

pub fn is_hexadecimal_color_code(color_code: &str) -> bool {
	let re = Regex::new(r"^#([0-9A-Fa-f]{3}|[0-9A-Fa-f]{6})$").unwrap();
	re.is_match(color_code)
}
