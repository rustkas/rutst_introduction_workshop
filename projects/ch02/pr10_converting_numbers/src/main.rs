#![allow(unused)]

fn main() {
    let multiply_result = multiply(-90, 10);
    println!("Multiplay = {}", multiply_result);

    let divide_result = divide(-90, 10);
    println!("Divade = {}", divide_result);
}


fn multiply(x: i64, y: u8) -> i64 {
    return x * (y as i64);
}

fn divide(x: i32, y: u16) -> f64 {
    return x as f64 / y as f64;
}