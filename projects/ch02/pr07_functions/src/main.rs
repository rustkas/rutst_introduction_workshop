fn main() {
    let answer = multiply_both(1.1, 2.2);
    println!("1.1 times 2.2 is {}", answer);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}