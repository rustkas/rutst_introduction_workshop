fn main() {
    let years = get_years();
    print_years(years.clone());
    print_years(years);
}

fn get_years() -> Vec<i32> {
    let years = vec![1995, 2000, 2005, 2010];
    return years;
}

fn print_years(years: Vec<i32>) -> Vec<i32> {
    for year in years.iter() {
        println!("Year: {}", year);
    }
    return years;
}
