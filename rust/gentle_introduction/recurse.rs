fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    for i in 0..5 {
        println!("Factorial {}: {}", i, factorial(i));
    }
}
