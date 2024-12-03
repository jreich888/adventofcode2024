
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{absolute, Path}, str::SplitAsciiWhitespace,
};
use rand::Rng;
use std::char;
use std::cmp::Ordering;
use std::io;
use std::collections::HashMap;
use regex::Regex;


// NOTE: this was practice code doing aoc 2023 Day 1

fn preprocess_line(line: String) -> String {
    let mut working = String::from(line.clone());



    working = working.replace("zero","0")
        .replace("one","1")
        .replace("two","2")
        .replace("three","3")
        .replace("four","4")
        .replace("five","5")
        .replace("six","6")
        .replace("seven","7")
        .replace("eight","8")
        .replace("nine","9");
    // if !line.eq(&working) {
        // println!("line has been modified: from {line} to {working}");
    // }
    return working;
}

fn findfirstnum(line: String) -> u32 {

    let number_map: HashMap<&str, u32> = HashMap::from([
        ("zero",0),
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);


    // find first ascii num index
    let mut idx = line.find(char::is_numeric).unwrap_or(999999);
    let ss = line.get(idx..idx+1).unwrap_or("0");
    let mut val = String::from( ss ).parse().expect("badparse");

    // iterate all the strings to find the earliest
    for each in number_map  {
        let s = each.0;
        let v = each.1;
        let i = line.find(s);
        if i.is_some_and(|x| x <= idx) {
            idx = i.unwrap();
            val = v;
        }
    }

    println!("Found first number at index {} value {} ", idx, val );

    return val;
}

fn findlastnum(line: String) -> u32 {

    let number_map: HashMap<&str, u32> = HashMap::from([
        ("zero",0),
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);


    // find first ascii num index
    let idx_opt = line.rfind(char::is_numeric);
    let mut val = 0;
    let mut idx = 0;
    if idx_opt.is_some() {
        idx = idx_opt.unwrap();
        let ss = line.get(idx..idx+1).expect("bad get");
        val = String::from( ss ).parse().expect("badparse");
        // println!("last index of num is idx {} {}", idx, val)
    }

    // let mut val = String::from( ss ).parse().expect("badparse");

    // iterate all the strings to find the earliest
    for each in number_map  {
        let s = each.0;
        let v = each.1;
        let i = line.rfind(s);
        if i.is_some_and(|x| x >= idx) {
            // println!("last index of updated for {s} num is idx {} {}", idx, val);
            idx = i.unwrap();
            val = v;
        }
    }

    println!("Found last number at index {} value {} ", idx, val );

    return val;
}

