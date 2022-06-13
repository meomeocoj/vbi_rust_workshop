// Count number the sub string in the paragraph or sentences
use std::io;

fn count_sub_str() -> u32 {
    let mut number = 0;

    number
}

fn main() {
    let paragraph = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal");

    let mut sub_str = String::new();

    io::stdin().read_line(&sub_str);

    println!("{}", paragraph);
}
