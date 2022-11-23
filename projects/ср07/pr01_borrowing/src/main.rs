
fn print_years(years: &Vec<i32>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }
}


fn main() {
    let years = vec![1990, 1995, 2000, 2010];

    print_years(&years);
    print_years(&years);
}


