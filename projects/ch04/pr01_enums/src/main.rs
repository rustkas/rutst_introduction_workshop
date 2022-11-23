#![allow(unused)]

enum Color {
    Green,
    Yellow, 
    Red,
    Custom {red: u8, green: u8, blue: u8},
    Custom2(u8,u8,u8)
}

fn main() {
    let go = Color::Green;
    let stop = Color::Red;
    let slow_down = Color::Yellow;
    let purple: Color = Color::Custom {
        red: 100, green: 0, blue: 250
    };

    let purple: Color = Color::Custom2(100, 0, 250);


    let curent_color = Color::Yellow;

    match curent_color {
        Color::Green => {
            println!("It was green!");
        }
        Color::Yellow => {
            println!("It was yellow!");
        }
        _ => ()
    }
}
