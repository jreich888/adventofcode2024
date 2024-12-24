// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use core::num;
use std::{collections::{btree_map::Keys, vec_deque, BTreeMap, HashMap, HashSet, VecDeque }, hash::Hash, i32::MAX, str::FromStr};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
use lazy_static::lazy_static;
use multimap::MultiMap;

#[derive(Debug,PartialEq,Clone)]
struct HalfAdder {
    in_xor: String,
    in_and: String,
    carry_in: String,
    carry_out: String,
    carry_mid: String,
    sum: String
}

impl HalfAdder {
    pub fn new() -> HalfAdder {
        let s = HalfAdder{
            in_xor : String::new(),
            in_and : String::new(),
            carry_in : String::new(),
            carry_out : String::new(),
            carry_mid : String::new(),
            sum : String::new()
        };
        return s;

    }
    
}

const AND: &str = "AND";
const OR: &str = "OR";
const XOR: &str = "XOR";

fn do_operations(work: &mut HashMap<String,(String,String,String)>, 
                 values: &mut HashMap<String,i32>) {
    while work.len() > 0 {
        println!("Loop: work len={}", work.len());
        let mut to_remove = String::new();
        let keys = work.keys().clone();
        for (k,v) in work.iter() {
            let (v1, op, v2) = work.get(k).unwrap();
            let val1 = values.get(v1);
            let val2 = values.get(v2);

            if val1.is_some() && val2.is_some() {
                let result: i32 = if op.eq(AND) {
                    val1.unwrap() & val2.unwrap()
                } else if op.eq(OR) {
                    val1.unwrap() | val2.unwrap()
                } else {
                    val1.unwrap() ^ val2.unwrap()
                };
                // println!("calculated {v1} {} {op} {v2} {} = {res} is {result}", val1.unwrap(), val2.unwrap() );
                values.insert((*k).clone(), result);
                // remove from work map
                // work.remove(k);
                to_remove = k.clone();
                break;
            }

        }
        if !to_remove.is_empty() { work.remove(&to_remove);}
    }
}

fn get_value(values: &mut HashMap<String,i32>, letter: &String) -> u64 {
    let mut value = 0u64;
    for (n,v) in values.iter() {
        println!("val {n}={v}" );

        if *v==1 && n.starts_with(letter) {
            let nbits: i32 = n.replace(letter,"").parse().unwrap();
            value += 1<<nbits;
        }
    }
    return value;
}

fn swap(opmap: &mut HashMap<String,(String,String,String)>, 
        k1: &str, k2: &str, swaps: &mut Vec<String> ) 
{
    let v1 = opmap.remove(k1).unwrap();
    let v2 = opmap.remove(k2).unwrap();
    opmap.insert(k1.to_string(),v2);
    opmap.insert(k2.to_string(),v1);
    swaps.push(k1.to_string());
    swaps.push(k2.to_string());
}



fn check_bits(values: &mut HashMap<String,i32>, bit_num: i32, corr_result: u64) -> bool {
    let x_bit = values.get(&format!("x{:02}", bit_num)).unwrap();
    let y_bit = values.get(&format!("y{:02}", bit_num)).unwrap();
    let z_bit = values.get(&format!("z{:02}", bit_num)).unwrap();

    let c_bit = ((corr_result >> bit_num) & 0x1) as i32;

    let correct = *z_bit == c_bit;
    println!("bit {bit_num}::: x {x_bit} y {y_bit} z {z_bit} corr {c_bit} {correct}");
    return correct;
}

fn ops_for_res(ops: &mut HashMap<String, (String,String,String)>, res: &String, known_good: &Vec<String>) -> Vec<String> {
    let mut list_of_results: Vec<String> = Vec::new();

    // look for operations that lead to the value res
    for (k,v) in ops.clone().iter() {
        if res.eq(k) {
            if ! known_good.contains(k) {
                println!("found in chain: {k} {:?}", v);
                list_of_results.push((*k).clone());
            }
            list_of_results.extend(ops_for_res(ops, &v.0, known_good));
            list_of_results.extend(ops_for_res(ops, &v.2, known_good));
        }
    }

    return list_of_results;

}

fn find_operation(opmap: &mut HashMap<String, (String,String,String)>,
    in1:String, op:&str, in2: String)  -> String 
{
    for (out, (i1,o,i2)) in opmap {
        if op.eq(o) {
            if (in1.eq(i1) && in2.eq(i2)) ||
                (in1.eq(i2) && in2.eq(i1)) { return out.clone(); }
        }
    }
    return String::new();

}



pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut score = 0u64;

    let mut values : HashMap<String,i32> = HashMap::new();
    // let mut operations : VecDeque<(String,String,String,String)> = VecDeque::new();
    let mut opmap : HashMap<String, (String,String,String)> = HashMap::new();

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

            // operations.push_back( (v1.to_string(),op.to_string(),v2.to_string(),res.to_string()) );
            opmap.insert( res.to_string(), (v1.to_string(),op.to_string(),v2.to_string()) );
        }
    }

    for (n,v) in values.iter() {
        println!("val {n}={v}" );
    }
    for (res, (v1,op,v2)) in opmap.clone().iter() {
        println!("operation {v1} {op} {v2} = {res}" );
    }


    // early swaps
    // swap z06 and dgh
    let mut swaps: Vec<String> = Vec::new();
    swap(&mut opmap, "z06", "dhg", &mut swaps);
    swap(&mut opmap, "brk", "dpd", &mut swaps);
    swap(&mut opmap, "z23", "bhd", &mut swaps);
    swap(&mut opmap, "z38", "nbf", &mut swaps);


    // dup our operations to work on 
    let mut work = opmap.clone();

    do_operations(&mut work, &mut values);

    println!("work is len={}", work.len());
    // calculate values
    score = 0;

    let x = get_value(&mut values, &"x".to_string());
    let y = get_value(&mut values, &"y".to_string());
    let z = get_value(&mut values, &"z".to_string());
    let mut maxz = 0i32;

    // find num bits in Z 
    for k in opmap.keys() {
        if k.starts_with("z") {
            let n: i32 = k.replace("z", "").parse().unwrap();
            maxz = maxz.max(n);
        }

    }
    println!("maxz is {maxz}");

    for (n,v) in values.iter() {
        // println!("val {n}={v}" );

        // if *v==1 && n.starts_with("z") {
        //     let nbits: i32 = n.replace("z","").parse().unwrap();
        //     score += 1<<nbits;
        // }


    }

    println!("x={x}");
    println!("y={y}");
    println!("z={z}");
    println!("x+y={}",x+y);

    let test = x+y;
    let mut known_good: Vec<String> = Vec::new();
    let mut ha_map: HashMap<i32,HalfAdder> = HashMap::new();
    let mut last_carry_out = String::new();

    for n in 0..maxz {
        let mut ha = HalfAdder::new();

        let correct = check_bits(&mut values, n, test);
        // let correct = check_bits(&mut values, 1, test);
        // let correct = check_bits(&mut values, 2, test);
        let key = format!("z{n:02}");
        let ops_for = ops_for_res(&mut opmap, &key, &known_good);
        if correct {
            known_good.extend(ops_for.clone());
        }
        println!("ops for {key}: {:?}", ops_for);

        // we know the output
        ha.sum = format!("z{n:02}");
        ha.carry_in = last_carry_out;
        // find the inputs
        ha.in_xor = find_operation(&mut opmap, format!("x{n:02}"), XOR, format!("y{n:02}"));
        ha.in_and = find_operation(&mut opmap, format!("x{n:02}"), AND, format!("y{n:02}"));

        ha.carry_mid = find_operation(&mut opmap, ha.in_xor.clone(), AND, ha.carry_in.clone());
        ha.carry_out = find_operation(&mut opmap, ha.in_and.clone(), OR, ha.carry_mid.clone());

        ha.sum = find_operation(&mut opmap, ha.in_xor.clone(), XOR, ha.carry_in.clone());

        if n==0 { ha.carry_out = ha.in_and.clone(); }

        println!("halfadder {n} {:?}", ha.clone());

        last_carry_out = ha.carry_out.clone();
        ha_map.insert(n,ha.clone());

        // Tests for this half adder
        // look for naming problem
        if !ha.sum.starts_with("z") || 
            ha.in_xor.starts_with("z") ||
            ha.in_and.starts_with("z") ||
            ha.carry_mid.starts_with("z") ||
            ha.carry_out.starts_with("z") {
            println!("PROBLEM NAMING for {n} {:?}", ha);
        }



        if n==0 { continue; }   // 0 is good, an breaks so many rules

        let sum_op = opmap.get(&ha.sum).unwrap();
        let ok1 = sum_op.1.eq(XOR);
        let ok2 = sum_op.0.eq(&ha.in_xor) || sum_op.2.eq(&ha.in_xor);
        // let ok2 = check_input(&mut opmap, ha.sum, ha.in_xor, ha.carry_in );
        if !ok1 { 
            let op = opmap.get(&ha.sum).unwrap();
            println!("PROBLEM {n} {} wrong operand {:?}", ha.sum, op);
        }
        if !ok2 { 
            let op = opmap.get(&ha.sum).unwrap();
            println!("PROBLEM {n} {} wrong inputs {:?}", ha.sum, op);
        }
        

    // let ops_for_1 = ops_for_res(&mut operations, &"z01".to_string());
    // println!("ops for z01: {:?}", ops_for_1);
    }

    swaps.sort();
    let swap_key = format!("swaps were: {:?}", swaps).replace(" ","").replace("\"","");
    println!("swap key is {swap_key}");

    score = z;

    return score as u64;
}