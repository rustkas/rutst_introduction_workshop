#![allow(unused)]

fn main() {
    let should_we_go_fast = true;
    let should_we_go_slow = false;
    let should_we_go_slow2 = !true;
    
    println!("true numeric value is {}", should_we_go_fast as u8);
    println!("false numeric value is {}", should_we_go_slow as u8);
    println!("false numeric value is {}", should_we_go_slow2 as u8);
    println!("1 == 2 {}", 1 == 2);

    let cats = 3;

    if cats > 1 {
        println!("Multiple cats");
    } else if cats > 1_000 {
        println!("Too many cats");
    }
     else {
        println!("Need more cats!");
    }
    
}
