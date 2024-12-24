// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use core::num;
use std::{collections::{vec_deque, BTreeMap, HashMap, HashSet, VecDeque }, hash::Hash, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
use lazy_static::lazy_static;
use multimap::MultiMap;

fn do_operations(work: &mut VecDeque<(String,String,String,String)>, 
                 values: &mut HashMap<String,i32>) {
    while work.len() > 0 {
        println!("Loop: work len={}", work.len());

        let (v1, op, v2, res) = work.pop_front().unwrap();

        let val1 = values.get(&v1);
        let val2 = values.get(&v2);

        if val1.is_some() && val2.is_some() {
            let result: i32 = if op.eq("AND") {
                val1.unwrap() & val2.unwrap()
            } else if op.eq("OR") {
                val1.unwrap() | val2.unwrap()
            } else {
                val1.unwrap() ^ val2.unwrap()
            };
            println!("calculated {v1} {} {op} {v2} {} = {res} is {result}", val1.unwrap(), val2.unwrap() );
            values.insert(res, result);


        } else {
            // put it back
            work.push_back((v1, op, v2, res));
        }




    }
}

pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut score = 0u64;

    let mut values : HashMap<String,i32> = HashMap::new();
    let mut operations : VecDeque<(String,String,String,String)> = VecDeque::new();

    for l in lines {
        let s: Vec<&str> = l.split_ascii_whitespace().collect();
        if s.len()>0 && s[0].contains(":") {
            // initial state
            // let s: Vec<&str> = l.split(": ").collect();
            let var = s[0].replace(":", "");
            let val: i32 = s[1].parse().unwrap();
            // println!("init value {var} = {val}");

            values.insert(var.to_string(),val);
        }
        if s.len()>3 && s[3].contains("->") {
            // operation
            // let s: Vec<&str> = l.split_ascii_whitespace().collect();
            let v1 = s[0];
            let op = s[1];
            let v2 = s[2];
            let res = s[4];
            // println!("parse op {v1} {op} {v2} = {res}" );

            operations.push_back( (v1.to_string(),op.to_string(),v2.to_string(),res.to_string()) );
        }
    }

    for (n,v) in values.iter() {
        println!("val {n}={v}" );
    }
    for (v1,op,v2,res) in operations.clone().iter() {
        println!("operation {v1} {op} {v2} = {res}" );
    }


    // dup our operations to work on 
    let mut work = operations.clone();

    do_operations(&mut work, &mut values);

    println!("work is len={}", work.len());
    // calculate values
    score = 0;
    for (n,v) in values.iter() {
        println!("val {n}={v}" );

        if *v==1 && n.starts_with("z") {
            let nbits: i32 = n.replace("z","").parse().unwrap();
            score += 1<<nbits;
        }
    }


    return score as u64;
}