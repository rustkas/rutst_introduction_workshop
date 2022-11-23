#![allow(unused)]

fn main() {
    let mut years: Vec<i32> = vec![1990, 1995];
    let mutable_years: &mut Vec<i32> = &mut years;

    let length = mutable_years.len();
    mutable_years.clear();


    let mut years: Vec<i32> = vec![1990, 1995];
    let years2: &mut Vec<i32> = &mut years;
    let years3: &mut Vec<i32> = &mut years;
    let years4: &mut Vec<i32> = &mut years;


    let mut years: Vec<i32> = vec![1990, 1995];

    let years2: &mut Vec<i32> = &mut years;
    let years3: &Vec<i32> = &years;
}
