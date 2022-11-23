#![allow(unused)]

struct Releases<'y> {
    years: &'y [i64],
    eighties: &'y [i64],
    nineties: &'y [i64],
}

fn main() {
    let years: Vec<i64> = vec![1980, 1985, 1990, 1990, 2000, 2005, 2010];

    let eighties: &[i64] = &years[0..2];
    let nineties: &[i64] = &years[2..4];

    println!("We have {} years in the nineties", nineties.len());


    let releases: Releases<'_> = {
        let all_years: Vec<i64> = vec![1980, 1985, 1990, 1990, 2000, 2005, 2010];
        jazz_releses(&all_years)
    };

    let eighties = releases.eighties;
    
}

fn jazz_releses<'a>(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];
    Releases {
        years,
        eighties,
        nineties,
    }
}
