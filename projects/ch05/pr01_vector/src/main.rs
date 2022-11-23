#![allow(unused)]

fn main() {
    let mut years: Vec<i32> = vec![1995, 2000, 2005];
    years.push(2010);
    years.push(2015);
    dbg!(&years);

    println!("Number of years: {}", years.len());
}
