use std::char;
use regex::Regex;



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