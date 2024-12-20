// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::{collections::{vec_deque, HashMap, HashSet, VecDeque}, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
// use multimap::MultiMap;


fn test_can_make(design: &String, towels: &mut Vec<String>) -> bool {
    // println!("  test can make design: {design}");

    for t in towels.clone() {
        // can this extend it?
        if design.eq(&t) { 
            // println!("     FULL MATCH {t}");
            return true;
        }
        if design.starts_with(t.as_str()) {
            // println!("     match {t}");
            // recurse, can we find the remainder to match?
            let subdesign = design.clone().split_off(t.len());
            if test_can_make(&subdesign, towels) {
                return true;
            }
        }
    }

    return false;
}





fn test_count_ways_p2(design: &String, towels: &mut Vec<String>, match_set: &mut HashMap<String,i64>) -> i64 {
    // println!("  test_count_ways_p2 design: {design}");

    if (match_set.contains_key(design)) { return *match_set.get(design).unwrap(); }


    let mut count: i64 = 0;

    for t in towels.clone() {
        // can this extend it?
        if design.eq(&t) { 
            // println!("     FULL MATCH {t}");
            count += 1;
        } else if design.starts_with(t.as_str()) {
            // println!("     match {t}");
            // recurse, can we find the remainder to match?
            let subdesign = design.clone().split_off(t.len());
            let ways = test_count_ways_p2(&subdesign, towels, match_set);
            count += ways;
            // println!("     match {t} count {ways}");
        }
    }
    // println!("  test_count_ways_p2 design: {design} count {count}");
    match_set.insert(design.clone(), count);

    return count;
}


pub fn process_lines(lines:Vec<String>) -> u64 {
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers

    let mut count = 0;
    let mut parsing_designs = false;
    let mut towels: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();
    let mut match_set: HashMap<String,i64> = HashMap::new();

    for l in lines {

        if l.trim().is_empty() { parsing_designs = true; continue; }

        if !parsing_designs {
            // parse towel
            towels = l.split_ascii_whitespace().map(|s| s.replace(",", "")).collect();
        } else {
            designs.push(l.trim().to_string());
        }
    }

    println!("towels: {:?}", towels);
    println!("designs: {:?}", designs);

    let mut score = 0;

    for d in designs {
        println!("Testing design {d}");
        // let can_make = test_can_make(&d, &mut towels);
        // println!("Testing design {d} result {can_make}");
        // if (can_make) { score+=1; }
        let ways = test_count_ways_p2(&d, &mut towels, &mut match_set);
        println!("Testing design {d} result {ways} ways");
        score += ways;
    }
    // unsafe { score = LOW_SCORE; }


    // let good_cells = count_cells_on_best_paths(&mut map, start_pos, end_pos);
    // println!("good cells count is {good_cells}");


    return score as u64;
}