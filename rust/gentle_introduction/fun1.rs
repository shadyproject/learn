fn sqr(x: f64) -> f64 {
    x * x
}

//absolute value
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

//ensure a number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);

    let num = -1.0;
    let abs1 = abs(num);
    println!("absolute value of {} is {}", num, abs1);

    let min = 5.0;
    let max = 18.0;
    let clamp_min = 1.0;
    let clamp_max = 99.0;
    let dont_clamp = 7.0;
    println!("clamping {}: {}", clamp_min, clamp(clamp_min, min, max));
    println!("clamping {}: {}", clamp_max, clamp(clamp_max, min, max));
    println!("clamping {}: {}", dont_clamp, clamp(dont_clamp, min, max));
}
