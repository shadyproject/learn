//match_switch.rs
use std::env;

fn main() {
    let input = env::args().nth(1).expect("Please enter a number");
    let n: i32 = input.parse().expect("That's not a number");

    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many"
    };

    println!("You entered {}", text);
}
