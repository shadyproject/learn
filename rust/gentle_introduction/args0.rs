//args0.rs
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    //skip the program name
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        //we have args
        for a in args {
            println!("'{}'", a);
        }
    } else {
        println!("No arguments");
    }
}
