//string3_match.rs
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";

    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("Couldn't find the greeting, Товарищ")
    };

    //if we don't care about the outcome of not finding it...
    if let Some(idx) = multilingual.find('п') {
        println!("Russian hi {}", &multilingual[idx..]);
    }
}
