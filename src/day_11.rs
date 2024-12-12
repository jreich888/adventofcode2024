// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashMap;




fn process_stone(n:u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }
    // let mut nstr = format!("{n}");
    let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
    //  println!("  digits: {n} {num_digits}");
    
    if num_digits % 2 == 0 {
        // split the number
        let split_factor = 10u64.pow(num_digits/2);
        let n1 = n / split_factor;
        let n2 = n % split_factor;
        // println!("  split: {n} {n1}, {n2}");
        return vec![n1,n2];
    } else {
        // multiply by 2024
        return vec![n*2024];
    }

}



pub fn count_single_stone_loops(s: u64, loops: i32, stone_cache: &mut HashMap<(u64,i32),u64> ) -> u64 {


    if loops == 0 { return 1 };

    let mut result = 0u64;

     {

    let cacheval = stone_cache.get(&(s,loops));
    if cacheval.is_some() { return *cacheval.unwrap(); }

    // process a loop
    let new_vals = process_stone(s);
    for ns in new_vals {
        result += count_single_stone_loops(ns, loops-1, stone_cache);
    }
    // cache the value
    stone_cache.insert( (s,loops), result );
    }
    
    return result;


}


pub fn count_stone_loops(stones:&Vec<u64>, loops: i32, print_log: bool) -> u64 {

    // let final_count = 0u64;
    const MAX_SUBSIZE : usize = 10000000;

    let mut work_stones = stones.clone();
    for loop_num in 1..loops+1 {

        if work_stones.len() > MAX_SUBSIZE {
            let (a1, a2)  = work_stones.split_at(work_stones.len()/2);
            let v1 = Vec::from(a1);
            let v2 = Vec::from(a2);
            // println!("split vec {:?} into {:?} and {:?}", work_stones, v1, v2);
            let remaining_loops = loops - loop_num + 1;

            let subcount =  count_stone_loops(&v1, remaining_loops, false)
                + count_stone_loops(&v2, remaining_loops, false);
                if print_log { println!(" LOOP {loop_num} stones count {}", subcount);    }
            return subcount;
        }


        let mut new_stones: Vec<u64> = Vec::new();

        for n in work_stones.clone() {
            let updated_stones = process_stone(n);
            // println!("  stone {n} turned into {:?}", updated_stones);
            new_stones.extend(updated_stones);
        }
        if print_log {
            println!(" LOOP {loop_num} stones count {}", new_stones.len());
        }
        // println!(" LOOP {loop_num} stones count {} {:?}", new_stones.len(), new_stones);
        work_stones = new_stones;
    }
    return work_stones.len() as u64;

}


pub fn process_lines(lines:Vec<String>) -> u64 {

    let start_line  =  &lines[0];

    // let mut stones: Vec<u64> = Vec::new();

    let stones: Vec<u64> = start_line
        .split_ascii_whitespace()
        .map(|s| {s.parse().expect("parse num")})
        .collect();
    println!("stones[{}]: {:?}", stones.len(), stones);

    // let length = count_stone_loops(&stones, 42, true);

    let mut stone_cache: HashMap< (u64,i32), u64 > = HashMap::new();

    let loops = 75;
    let mut total = 0u64;
    for n in stones.clone() {
        let subtotal = count_single_stone_loops(n,loops,&mut stone_cache);
        println!("count for stone {n} loops {loops} is {subtotal}");
        total += subtotal;
    }

    println!("final stone_cache size is {}", stone_cache.len());

    // let loop_count = 75;
    // for loop_num in 1..loop_count+1 {
    //     let mut new_stones: Vec<u64> = Vec::new();

    //     for n in stones.clone() {
    //         let updated_stones = process_stone(n);
    //         // println!("  stone {n} turned into {:?}", updated_stones);
    //         new_stones.extend(updated_stones);
    //     }
    //     println!(" LOOP {loop_num} stones count {}", new_stones.len());
    //     // println!(" LOOP {loop_num} stones count {} {:?}", new_stones.len(), new_stones);
    //     stones = new_stones;
    // }


    
   return total as u64;

}