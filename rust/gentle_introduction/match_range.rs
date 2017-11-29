//match_range.rs
use std::env;

fn main() {
    let input = env::args().nth(1).expect("Please enter a number");
    let n: u32 = input.parse().expect("That's not a number");

    let text = match n {
        0...3 => "small",
        4...5 => "medium",
        _ => "large"
    };

    println!("Size: {}", text);
}
