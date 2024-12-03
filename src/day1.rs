
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{absolute, Path},
};
use std::char;
use std::cmp::Ordering;
use std::io;
use std::collections::HashMap;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}











fn process_lines(lines:Vec<String>) -> u64 {

    let mut one: Vec<u64> = Vec::new();
    let mut two: Vec<u64> = Vec::new();


    let mut sum = 0;
    for orig in lines {
        println!("orig: {orig}");

        // let nums = orig.split_ascii_whitespace().map(|s| s.parse());
        let nums = orig.split_ascii_whitespace();
        for n in nums {  println!("  n: {n}"); }

        let mut nums = orig.split_ascii_whitespace().map(|s| s.parse::<u64>());
        let a = nums.next().unwrap().unwrap();
        let b = nums.next().unwrap().unwrap();
        // for n in nums {  println!("  n: {}", n.expect("parserr")); }
        println!("parsed {a} // {b}");

        one.push(a);
        two.push(b);
    }

    // for n in one { println!("one: {n}")};
    // for n in two { println!("two: {n}")};

    // one.sort();
    // two.sort();
    // for n in one.clone() { println!("one: {n}")};

    let mut two_count: HashMap<u64, u64> = HashMap::new();

    for n in two.clone() { 
        let mut val= 1;
        val = two_count.get(&n).unwrap_or(&0u64) + 1;
        two_count.insert(n,val);
        // println!("two: {n}")
    };

    // while !one.is_empty() {
    //     let a = one.pop().expect("pop one") as i64;
    //     let b = two.pop().expect("pop two") as i64;
    //     let diff = (a-b).abs();
    //     println!( " {a} {b} {diff}");
    //     sum += diff;
    // }
    for n in one.clone() { 
        let factor = two_count.get(&n).unwrap_or(&0u64);
        let distance = n * factor;
        sum += distance;
        println!("one: {n} {factor} {distance} {sum}")
    };

    return sum as u64;

}

fn main() {
    println!("Hello, world!");
    let lines = read_lines("./input/sample1.txt");

    // aicode(lines);
    // return;

    let lines = read_lines("./input/input_1.txt");
    let result = process_lines(lines);

    println!("Result of file is: {result} ");



}



// Testing code generate by ChatGPT
fn aicode(lines : Vec<String>) {
        // Read input lists
        let file = File::open("input/sample1.txt");
        let mut lines = BufReader::new(file.expect("asdf")).lines();

        // Read the first list
        let left_list: Vec<i32> = lines
            .next()
            .expect("Expected a line for the left list")
            .expect("asdf")
            .split_whitespace()
            .map(|x| x.parse().expect("Expected an integer"))
            .collect();
        for n in left_list.clone() { println!("one before: {n}")};
        
        // Read the second list
        let right_list: Vec<i32> = lines
            .next()
            .expect("Expected a line for the right list")
            .expect("asdf")
            .split_whitespace()
            .map(|x| x.parse().expect("Expected an integer"))
            .collect();
    
        // Sort both lists
        let mut sorted_left = left_list.clone();
        sorted_left.sort_unstable();
        
    for n in sorted_left.clone() { println!("one: {n}")};


        let mut sorted_right = right_list.clone();
        sorted_right.sort_unstable();
        
        // Calculate total distance
        let total_distance: i32 = sorted_left
            .iter()
            .zip(sorted_right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        
        println!("Total distance: {}", total_distance);
    
}