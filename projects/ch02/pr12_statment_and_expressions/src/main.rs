#![allow(unused)]

fn main() {
    let cats = 3;
    let message = if cats > 1 {
        "Multiple cats"
    } else if cats > 1_000 {
        "Too many cats"
    } else {
        "Need more cats!"
    };

    println!("{message}");
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}

fn multiply_both2(x: f64, y: f64) -> f64 {
    x * y
}
