// Count number the sub string in the paragraph or sentences
use std::io;

fn count_sub_str(paragraph: &str, sub_str: &str) -> u32 {
    let mut number = 0;
    let words = paragraph.split_whitespace();
    for word in words {
        if word.eq(sub_str) {
            number += 1;
        }
    }
    number
}

fn main() {
    let paragraph = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal";

    let mut sub_str = String::new();
    println!("Please enter the word");
    io::stdin()
        .read_line(&mut sub_str)
        .expect("Fail to readline");

    let sub_str = sub_str.trim();

    println!(
        "Number of substring {}",
        count_sub_str(&paragraph, &sub_str)
    );
}
