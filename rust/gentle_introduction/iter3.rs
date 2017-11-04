// iter3.rs
fn main() {
    println!("iter");
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("slice");
    // slices are converted implicitly to iterators ...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}
