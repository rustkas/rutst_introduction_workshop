#![allow(unused)]

fn main() {
    let mut years = vec![1999, 2000];
    let years_ref = &years;

    let x = &mut years;
    let slice: &[i32] = &years[1..3];

    
}
