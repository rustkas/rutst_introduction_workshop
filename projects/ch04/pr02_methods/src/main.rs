#![allow(unused)]

enum Color {
    Green,
    Yellow, 
    Red,
    Custom {red: u8, green: u8, blue: u8},
    Custom2(u8,u8,u8)
}

impl Color {
    fn rgb(color: Color) -> (u8, u8, u8) {
        (0,0,0)
    }

    fn new(r: u8, g: u8, b: u8) -> Self {
        Color::Custom2(r, g, b)
    }
}

fn main() {
    let red = Color::new(250,0,0);
    let purple = Color::new(100, 0, 250);
    let (r, g, b) = Color::rgb(purple);
}