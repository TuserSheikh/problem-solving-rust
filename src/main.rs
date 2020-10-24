mod string;

fn main() {
    // let reverse_word = string::reverse_word("Harry Potter");
    // println!("{:?}", reverse_word);

    // let is_panagram = string::panagram_check("The quick brown fox jumps over the lazy dog");
    // println!("{:?}", is_panagram);
    // let is_panagram = string::panagram_check("I’ve always wanted to use that spell!");
    // println!("{:?}", is_panagram);

    let missing_char_for_panagram =
        string::missing_char_for_panagram("The quick brown fox jumps over the lazy dog");
    if missing_char_for_panagram.len() > 0 {
        println!(
            "Missing character for panagram: {:?}",
            missing_char_for_panagram
        );
    } else {
        println!("No missing char, means its panagram");
    }

    let missing_char_for_panagram =
        string::missing_char_for_panagram("The quick brown fox jumps over the lazy");
    if missing_char_for_panagram.len() > 0 {
        println!(
            "Missing character for panagram: {:?}",
            missing_char_for_panagram
        );
    } else {
        println!("No missing char, means its panagram");
    }
}
