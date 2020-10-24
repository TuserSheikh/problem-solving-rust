mod string;

fn main() {
    // let reverse_word = string::reverse_word("Harry Potter");
    // println!("{:?}", reverse_word);

    let is_panagram = string::panagram_check("The quick brown fox jumps over the lazy dog");
    println!("{:?}", is_panagram);
    let is_panagram = string::panagram_check("I’ve always wanted to use that spell!");
    println!("{:?}", is_panagram);
}
