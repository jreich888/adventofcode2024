
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
};

pub mod day_16;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}








fn main() {
    println!("Hello, world!");
    let inputday : &str = "day16";
    let lines = read_lines(format!("./input/{inputday}/sample1.txt"));
    let lines = read_lines(format!("./input/{inputday}/sample2.txt"));
    // let lines = read_lines(format!("./input/{inputday}/test2.txt"));
    let lines = read_lines(format!("./input/{inputday}/input_1.txt"));

    let result = day_16::process_lines(lines);
    println!("Result of file is: {result} ");
}
