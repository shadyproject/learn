// sum1.rs
// idiomatic way to do this in rust
fn main() {
    let sum: i32 = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
}
