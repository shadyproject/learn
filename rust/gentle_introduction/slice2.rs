//slice2.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);

    println!("First: {:?}", first);
    println!("first {} {}", first.is_some(), first.is_none());
    println!("first {}", first.unwrap());

    let maybe_last = slice.get(5);
    let last = maybe_last.unwrap_or(&-1);
    println!("last: {}", last);
}
