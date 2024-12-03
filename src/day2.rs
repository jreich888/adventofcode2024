

fn is_safe(nums: Vec<i32>, allow_replace: bool ) -> bool {
    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;
    let mut dir = 0;
    let mut last = -1;
    for n in nums {
        if (last < 0) { last = n; continue; }
        let diff: i32 = n - last;
        if diff.abs() < MIN_DIFF { return false; }
        if diff.abs() > MAX_DIFF { return false; }
        let diffsign = diff / diff.abs();
        println!("  {last} {n} {diff} {dir} {diffsign}");
        if dir == 0 { dir = diffsign; }
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

    let mut one: Vec<u64> = Vec::new();
    let mut two: Vec<u64> = Vec::new();


    let mut sum = 0;
    for orig in lines {
        println!("orig: {orig}");
        let safe = is_safe_line(&orig, false);
        println!("orig: {orig} {safe}");
        println!("--");
        if safe { sum += 1; }

    }
    return sum as u64;

}

pub fn day2(lines:Vec<String>) {
    println!("Hello, world!");
    
    let result = process_lines(lines);

    println!("Result of file is: {result} ");

}
