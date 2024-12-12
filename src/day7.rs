// use regex::{bytes, Regex};
use std::collections::HashSet;





fn test_line( cal_formula : &Vec<i64> ) -> i64 {
    let test = false;

    let (result,inputs) = cal_formula.split_at(1);
    // let inputs = cal_formula[1..];

    println!("cal answer {:?} inputs {:?}", result, inputs);

    // Test all the operation combinations
    // if inputs[0] * inputs[1] == result[0] { 
        // We have 2 operations, so let's do some binary math
        let num_operations = inputs.len()-1;
        let two = 2u64;
        let op_mask_len = (two.pow(num_operations as u32));
        println!("num {num_operations} op_mask_len {op_mask_len}");
        for op_mask in 0..op_mask_len {
            let mut test_result = inputs[0];
            let mut work_op_mask = op_mask;
            for i in 1..(num_operations+1) {
                if work_op_mask & 0x01 == 1 {
                    test_result *= inputs[i];
                } else {
                    test_result += inputs[i];
                }
                work_op_mask = work_op_mask >> 1;
            }
            let matches = (test_result == result[0]);
            // println!("   matches {matches} test result {test_result} result {} op_mask 0x{:x}", result[0], op_mask);
            if matches { 
                println!("!!Found one: {:?}", cal_formula);
                return result[0]; 
            }
        }
    // }

    println!("  -- NO MATCH: {:?}", cal_formula);

    return 0;
}


fn test_line_p2( cal_formula : &Vec<i64> ) -> i64 {
    let test = false;

    let (result,inputs) = cal_formula.split_at(1);
    // let inputs = cal_formula[1..];

    println!("cal answer {:?} inputs {:?}", result, inputs);

    // Test all the operation combinations
    // if inputs[0] * inputs[1] == result[0] { 
        // We have 2 operations, so let's do some binary math
        let num_operations = inputs.len()-1;
        let three = 3u64;
        let op_mask_len = (three.pow(num_operations as u32));
        println!("num {num_operations} op_mask_len {op_mask_len}");
        for op_mask in 0..op_mask_len {
            let mut test_result = inputs[0];
            let mut work_op_mask = op_mask;

            // let mut debug_output : String = format!("{test_result} ");

            for i in 1..(num_operations+1) {
                let cur_op = work_op_mask % 3;
                match cur_op {
                    0 => { test_result += inputs[i];
                        // debug_output = format!("{debug_output} + {}", inputs[i]);
                    },
                    1 => {test_result *= inputs[i];
                        // debug_output = format!("{debug_output} * {}", inputs[i]);
                    },
                    _ => {
                        // concatenation operation
                        test_result = concat_nums(test_result, inputs[i]);
                        // debug_output = format!("{debug_output} || {}", inputs[i]);
                    }
                }
                // println!("   {debug_output} {test_result}");
                work_op_mask = work_op_mask / 3;
            }

            let matches = (test_result == result[0]);
            // println!("   {debug_output} {test_result}");
            // println!("   matches {matches} test result {test_result} result {} op_mask 0x{:x}", result[0], op_mask);
            if matches { 
                println!("!!Found one: {:?}", cal_formula);
                return result[0]; 
            }
        }
    // }

    println!("  -- NO MATCH: {:?}", cal_formula);

    return 0;
}


pub fn concat_nums(x: i64, y: i64) -> i64 {
    // concat x then y
    let base: i64 = 10;
    let positions = y.ilog10()+1;
    return x * 10i64.pow(positions) + y;
}

pub fn process_lines(lines:Vec<String>) -> u64 {

    // println!("concat_test 444 888 {}", concat_nums(444, 888));
    // println!("concat_test 3 777777 {}", concat_nums(3, 777777));
    // println!("concat_test 10101 5 {}", concat_nums(10101, 5));
    // return 0;

    let mut count : i64 = 0;

    // Lets process the map.  Input: guard is arrow (any 4 dirs), and obstacles are #
    // Convert input to Vec<Vec<i32>> with: const values above

    let mut input : Vec<Vec<i64>> = Vec::new();

    for l in lines {
            println!("line: {l}");
            let line_input = l.split_ascii_whitespace().map(|n| {
            // println!("{l} /// {n}");
            let x : i64 = n.
            trim_end_matches(":").
            parse().expect("parse error");
            x
        }).collect();
        input.push(line_input);
    }

    for i in input.clone() {
        // println!("input line: {:?}", i );
        let test_val = test_line_p2( &i );
        count += test_val;
    }




    return count as u64;

}