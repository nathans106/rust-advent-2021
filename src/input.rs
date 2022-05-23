use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub type DayId = i8;

pub fn parse_strs(day_num: &DayId) -> Vec<String> {
    let path = format!("./inputs/day{}.txt", day_num);
    read_lines(&path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect()
}

pub fn parse_vec<T>(day_num: &DayId) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
    T: std::str::FromStr,
{
    let path = format!("./inputs/day{}.txt", day_num);
    read_lines(&path)
        .unwrap()
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
