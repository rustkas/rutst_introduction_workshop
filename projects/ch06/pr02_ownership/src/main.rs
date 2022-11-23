fn main() {
    let years = get_years();
    dbg!(&years);
    let years2 = print_years(years);
    dbg!(&years2);
    let years3 = print_years(years2);
    dbg!(&years3);
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