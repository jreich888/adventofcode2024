
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
};

pub mod day2;
pub mod day3;
pub mod day4;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}








fn main() {
    println!("Hello, world!");
    let inputday : &str = "day04";
    let lines = read_lines(format!("./input/{inputday}/sample1.txt"));
    // let lines = read_lines("./input/" + inputday + "/sample2.txt");
    // let lines = read_lines("./input/" + inputday + "/input_1.txt");

    let result = day4::process_lines(lines);
    println!("Result of file is: {result} ");
}
