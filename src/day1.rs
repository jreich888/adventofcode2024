
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


fn is_safe(nums: Vec<i32>, allow_replace: bool ) -> bool {
    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;
    let mut dir = 0;
    let mut last = -1;
    for n in nums {
        if (last < 0) { last = n; continue; }
        let diff: i32 = n - last;
        let mut diffsign = 0;
        if diff!=0 { diffsign = diff / diff.abs() }
        println!("  {last} {n} {diff} {dir} {diffsign}");
        if dir == 0 { dir = diffsign; }

        // failure cases
        if diff.abs() < MIN_DIFF { return false; }
        if diff.abs() > MAX_DIFF { return false; }
        if diffsign != dir {return false; }

        last = n;
    }
    return true;

}

fn is_safe_line(line : &String, allow_replace: bool) -> bool {
    let nums = line.split_ascii_whitespace();
    let mut numvec: Vec<i32> = Vec::new();
    for n in nums {
        numvec.push(n.parse().expect("parse error"));
    }


        
    let initial_safe = is_safe(numvec.clone(), false);
    if initial_safe { return true; }

    // Ok, start to clone and split the lines
    for n in 0..numvec.len() {
        let mut tvec = numvec.clone();
        tvec.remove(n);
        let subsafe = is_safe(tvec, false);
        println!( "  subsafe {n} is {subsafe}");
        if subsafe { return true; }
    }

    return false;


}


pub fn process_lines(lines:Vec<String>) -> u64 {

    // let part1_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    
    let part2_re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut sum = 0;
    for orig in lines {
        println!("orig: {orig}");

        let mults: Vec<(&str,i32, i32)> = part2_re.
            captures_iter(&orig).
            map(|caps| {
                // println!("{}", caps.);
                let all = caps.get(0).expect("err x").as_str();
                // println!("found: {all}");
                let mut x = 0;
                let mut y = 0;
                if all.starts_with("mul") {
                    x = caps.get(1)
                        .unwrap()
                        .as_str().
                        parse().
                        expect("0");
                    y =  caps.get(2).expect("err y").as_str().parse().expect("parse x");
                }
                // let (_, [x,y]) = caps.extract();
                println!("found: {all} {x} {y} ");
                (all,x,y)
            }).collect();

            static mut running : bool = true;

            unsafe {
            for c in mults {
                if c.0.starts_with("don't(") { running = false }
                if c.0.starts_with("do(") { running = true }
                println!("  mult: {} running={running} {} {} ",c.0,c.1, c.2);
                if running {
                    sum += c.1 * c.2;
                }
            }
        }
        println!("--");
        // if safe { sum += 1; }

    }
    return sum as u64;

}