mod string;

fn main() {
    // let reverse_word = string::reverse_word("Harry Potter");
    // println!("{:?}", reverse_word);

    // let is_panagram = string::is_panagram("The quick brown fox jumps over the lazy dog");
    // println!("{:?}", is_panagram);

    // let missing_char = string::missing_char_for_panagram("Dobby is free");
    // println!("{}", missing_char);

    let is_hex_color_code = string::validate::is_hexadecimal_color_code("#000000");
    println!("{}", is_hex_color_code);
}
