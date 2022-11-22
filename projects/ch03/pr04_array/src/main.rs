#![allow(unused)]

fn main() {
    let mut years: [i32; 3] = [1995, 2000, 2005];
    dbg!(years);
    let first_year = years[0];
    let [_, second_years, third_year] = years;

    years[2] = 2010;
    dbg!(years);
    // years[x] = 2010;

    for year in years.iter() {
        println!("Next year: {}", year + 1);
    }

    let [year1, year2, year3] = years;
    
}
