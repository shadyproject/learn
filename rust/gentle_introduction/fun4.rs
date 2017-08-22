fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    println!("res is {}", res);
    modifies(&mut res);
    println!("res is {}", res);

    //IMPOTANT: THIS DOESN'T WORK
    //Uncomment to see the error warning to better understand
    //let mut_res = modifies(&mut res);
    //println!("res is {}", mut_res);
}
