use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::{collections::{HashMap, HashSet}, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
// use geo::{coord};
// use multimap::MultiMap;

#[derive(Debug,Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
    output: Vec<u8>,
    ip: usize
}


fn get_real_operand(computer: &mut Computer, coop: u8) -> i64 {
    return match coop { 
        0..=3 => coop as i64,
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => { assert!(false); 0 }
    }
}

const DEBUG:bool = false;

fn process_adv(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("ADV {co_op}"); }
    computer.a = computer.a / 2i64.pow(co_op as u32);
}
fn process_bdv(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("BDV {co_op}"); }
    computer.b = computer.a / 2i64.pow(co_op as u32);
}
fn process_cdv(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("CDV {co_op}"); }
    computer.c = computer.a / 2i64.pow(co_op as u32);
}
fn process_bxl(computer: &mut Computer,lit_op: u8) {
    if DEBUG { println!("BXL {lit_op}"); }
    computer.b = computer.b ^ lit_op as i64;
}
fn process_bst(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("BST {co_op}"); }
    computer.b = co_op % 8;
}
fn process_jnz(computer: &mut Computer,lit_op: u8) -> bool {
    if DEBUG { println!("JNZ {lit_op}"); }
    if (computer.a != 0) {
        computer.ip = lit_op as usize;
        return true;
    }
    return false;
}
fn process_bxc(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("BXC"); }
    computer.b = computer.b ^ computer.c;
}
fn process_out(computer: &mut Computer,co_op: i64) {
    if DEBUG { println!("OUT {co_op}"); }
    computer.output.push((co_op%8i64) as u8);
}

fn run_program(computer: &mut Computer) {
    // println!("instructions {:?}", computer.program);

    loop {
        if computer.ip >= computer.program.len() {
            if DEBUG { println!("HALT"); }
            break;
        }

        let inst = computer.program[computer.ip];
        let lit_op = computer.program[computer.ip+1];

        let co_op = get_real_operand(computer, lit_op);

        if DEBUG { println!("instruction {inst} lit_op {lit_op} co_op {co_op}"); }

        let mut skip_ip_incr = false;
        match inst  {
            0 => process_adv(computer,co_op),
            1 => process_bxl(computer,lit_op), // USES LITERAL, NOT COMBO
            2 => process_bst(computer,co_op),
            3 => skip_ip_incr = process_jnz(computer,lit_op), // USES LITERAL, NOT COMBO
            4 => process_bxc(computer,co_op),
            5 => process_out(computer,co_op),
            6 => process_bdv(computer,co_op),
            7 => process_cdv(computer,co_op),
            _ => ()
        }

        if !skip_ip_incr {
            computer.ip += 2;
        }

        // println!("computer: {:?}",computer);
    }

}


pub fn find_digits( digits_so_far: &mut Vec<HashSet<i8>>, to_find:i32 ) -> HashSet<i8> {
    let mut matches: HashSet<i8> = HashSet::new();

    let mut start_values: Vec<i64> = vec![0];
    for ii in digits_so_far.clone() {
        let tvec = start_values.clone();
        start_values.clear();
        for n in tvec {
            for jj in ii.clone() {
                start_values.push(n*8+jj as i64)
            }
        }
    }


    for n in start_values {
        for i in 0..8 {
            let a = n*8 + i;
            // let a = ((((((6*8) + 1)*8 + 1)*8 + 7)*8 + 6)*8 + 5)*8 + i;
            let b: i64 = (a%8) ^ 3;
            let c = a / 2i64.pow(b as  u32);
            let b = b ^ 5;
            let out = c ^ b;
            let out2 = out %8;
            // if out2 == 0 {
            // println!("i={i} a={a} amod8={} out={out} out2={out2}",a%8);
            if out2 == to_find as i64 { 
                // println!("match {out2} {to_find} {i}");
                matches.insert(i as i8); }
            // }
        }
    }

    

    digits_so_far.push(matches.clone());


    return matches;

}

pub fn process_lines(lines:Vec<String>) -> u64 {
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers


    let mut map : Vec<Vec<char>> = Vec::new();

    let mut computer = Computer{
        a:0, b:0, c:0, program: Vec::new(), output: Vec::new(), ip: 0
    };
    

    for l in lines {
        if l.starts_with("Register") {
            let reg_parse = Regex::new(r"Register (.): ([0-9]+)").unwrap();
            let caps = reg_parse.captures(&l).expect("parse reg");
            let reg_letter = caps.get(1).unwrap().as_str();
            let s = caps.get(2).unwrap().as_str();
            // println!("register s={s}");
            let reg_val: i64 = s.parse().unwrap();
            println!("register {reg_letter} = {reg_val}");
            match reg_letter {
                "A" => computer.a = reg_val,
                "B" => computer.b = reg_val,
                "C" => computer.c = reg_val,
                _ => assert!(false)
            }
        }
        if l.starts_with("Program") {
            let instructions = l.replace("Program: ","");
            println!("instructions: {instructions}");

            computer.program = instructions.split(",").map(|s| s.parse().unwrap()).collect();
        }
    }


    println!("computer is {:?}", computer);

    let mut out_len= 0usize;
    let prog_size= computer.program.len();
    
    let mut digits_so_far: Vec<HashSet<i8>> = Vec::new();

    for to_find in computer.program.clone().iter().rev() {

        println!("Finding {to_find}");
        let digs = find_digits(&mut digits_so_far, *to_find as i32);
        for d in digs { println!("digs {d}"); }
        println!("digits: {:?}", digits_so_far);
    }

    println!("digits: {:?}", digits_so_far);



    
    // all the possible values for A
    let mut all_a_values: Vec<i64> = vec![0];
    for ii in digits_so_far.clone() {
        let tvec = all_a_values.clone();
        all_a_values.clear();
        for n in tvec {
            for jj in ii.clone() {
                all_a_values.push(n*8+jj as i64)
            }
        }
    }


    
    let mut a_matches : Vec<i64> = Vec::new();
    let mut runcount = 0;
    let num_a_vals = all_a_values.len();
    println!("num of poss a value={}", num_a_vals);

    for n in all_a_values {
        // println!("Possible A value: {n}");

        let mut work_computer = computer.clone();
        work_computer.a = n;

        run_program(&mut work_computer);



        if work_computer.program == work_computer.output {
            println!("For A={n} we get matching output!");
            println!("program is {:?}", work_computer.program);
            println!("output  is {:?}", work_computer.output);
            a_matches.push(n);
        }

        runcount += 1;
        if runcount%10000 == 0 {
            let pct = runcount / num_a_vals;
            println!("tested {runcount} / {num_a_vals}  {pct}% a values.");
        }

    }

    println!("num of poss a value={}", num_a_vals);
    println!("a matches: {:?}", a_matches);
    println!("a matches: {:?}", a_matches.sort());


    return 0u64;

    // // digits_so_far.push( vec![6] );
    // println!("Finding 3");
    // let digs = find_digits(&mut digits_so_far, 3);
    // for d in digs { println!("digs {d}"); }
    // println!("Finding 5");
    // let digs = find_digits(&mut digits_so_far, 5);
    // for d in digs { println!("digs {d}"); }
    // println!("Finding 5");
    // let digs = find_digits(&mut digits_so_far, 5);
    // for d in digs { println!("digs {d}"); }


    let inputs = [6,1,1,7,6,5,7,4];

    let mut a_start = 0;
    for ii in inputs {
        a_start = a_start*8 + ii;
    }

    for i in 0..8 {
        let a = a_start*8 + i;
        // let a = ((((((6*8) + 1)*8 + 1)*8 + 7)*8 + 6)*8 + 5)*8 + i;
        let b: i32 = (a%8) ^ 3;
        let c = a / 2i32.pow(b as  u32);
        let b = b ^ 5;
        let out = c ^ b;
        let out2 = out %8;
        // if out2 == 0 {
        println!("i={i} a={a} amod8={} out={out} out2={out2}",a%8);
        // }
    }

    // return 0u64;


    // Part 2: override A with 117440
    let MILLION = 1000*1000;
    let BILLION = 1000*MILLION;
    let TRILLION = 1000*BILLION;

    println!("program is {:?}", computer.program);
    let start = 195*BILLION;
    let start = 
        6*8i64.pow(15) + 
        1*8i64.pow(14) + 
        1*8i64.pow( 13) + 
        7*8i64.pow( 12) +
        6*8i64.pow( 11) +
        5*8i64.pow( 10) +
        // 7*8i64.pow( 9) +
        // 4*8i64.pow( 8) +
        // 2*8i64.pow( 7) +
        0;
    for inp in start..start+200*MILLION {
        // let i = inp * 1000;
        let i = inp;
        if (i%100000==0) {
            // println!("loop {i} {}k {}M prog size {prog_size} out size {out_len}", i/1000, i/1000/1000);
        }

        let mut work_computer = computer.clone();
        work_computer.a = i;

        run_program(&mut work_computer);

        // println!("RUN {i}");
        // println!("program is {:?}", work_computer.program);
        // println!("output  is {:?}", work_computer.output);
        out_len = work_computer.output.len();
        // if (out_len == prog_size && work_computer.output[out_len-1] == computer.program[prog_size-1]) {
        //     println!("LAST DIGIT MATCHING");
        //     println!("program is {:?}    {i}", computer.program);
        //     println!("output  is {:?}", work_computer.output);
        
        // }

        if (i%1000000==0) {
            println!("loop {i} {}k {}M prog size {prog_size} out size {out_len}", i/1000, i/1000/1000);
            println!("output  is {:?}", work_computer.output);
        }


        if work_computer.program == work_computer.output {
            println!("For A={i} we get matching output!");
            println!("program is {:?}", work_computer.program);
            println!("output  is {:?}", work_computer.output);
            break;
        }
    }

    // let output = format!("{:?}", computer.output)
    //     .replace("[","").replace("]","").replace(" ", "");
    // println!("output is {output}");

    return 0u64 as u64;


}