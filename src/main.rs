
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
};

pub mod day2;
pub mod day3;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}








fn main() {
    println!("Hello, world!");
    let lines = read_lines("./input/day02/sample1.txt");
    // let lines = read_lines("./input/day02/sample2.txt");
    // let lines = read_lines("./input/day02/input_1.txt");

    let result = day2::process_lines(lines);
    println!("Result of file is: {result} ");
}
