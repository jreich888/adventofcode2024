// use regex::{bytes, Regex};


fn test_page_before(pagelist : Vec<i32>,first:i32,second:i32) -> bool {
    let firstpos =   pagelist.iter().position(|&p| p == first);
    let secondpos =   pagelist.iter().position(|&p| p == second);
    if firstpos.is_none() || secondpos.is_none() { return true; }
    let fp = firstpos.unwrap();
    let sp = secondpos.unwrap();
    // println!("positions: {first},{second}  :: {fp} {sp}");
    return fp < sp;
}

fn test_page_list(pagelist : Vec<i32>, rules : &Vec<(i32,i32)> ) -> bool {
    // for pn in pagelist.clone() { println!("page {pn}")};
    let mut is_good = true;
    for r in rules {
        is_good = is_good && test_page_before(pagelist.clone(),r.0,r.1);
        if !is_good { break; }
    }
    return is_good;
}

fn fix_pagelist_and_return_middle(pagelist : Vec<i32>, rules : &Vec<(i32,i32)> ) -> i32 {
    print!("fixing pagelist: {:?}", pagelist);
    // for pn in pagelist.clone() { print!("{pn} ")};
    
    // break our recursion
    if test_page_list(pagelist.clone(), rules) { 
        println!(" GOOD!!!");  return get_middle_page(pagelist.clone()) 
    }
    println!("");

    // for pn in pagelist.clone() { println!("page {pn}")};
    let mut is_good = true;
    for r in rules {
        is_good = is_good && test_page_before(pagelist.clone(),r.0,r.1);
        if !is_good { 
            // swap the violators.  We know both pages are present, so can directly unwrap
            let fp =   pagelist.iter().position(|&p| p == r.0).unwrap();
            let sp =   pagelist.iter().position(|&p| p == r.1).unwrap();
            let mut npl = pagelist.clone();
            let v = npl.remove(fp);
            npl.insert(sp, v);

            // recurse
            return fix_pagelist_and_return_middle(npl, rules);
        }
    }
    return -1;
}

fn get_middle_page(pagelist : Vec<i32>) -> i32 {
    let len = pagelist.len();
    return *pagelist.get(len/2).unwrap() as i32;
}




pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut correct_row_count = 0;
    let mut corrected_page_sums = 0;

    // Separate the rules from the page sets
    let rule_lines: Vec<&String>  = lines.iter().filter(|l| return l.contains("|")).collect();
    let page_lines: Vec<&String> = lines.iter().filter(|l| return l.contains(",")).collect();

    // process rules into vec of numbers
    let rules: Vec<(i32,i32)> = 
        rule_lines.iter().map(|s| {
            let mut ruliter = s.split("|").into_iter();
            ( ruliter.next().unwrap().parse().expect("parse before"),
              ruliter.next().unwrap().parse().expect("parse after") )
            // println!("found rule {before} {after}");
            // (before,after)
        } ).collect();
   

    // for rl in rule_lines { println!("rule: {rl}")};
    // for pl in page_lines { println!("page: {pl}")};
    for r in rules.clone() { println!("rule: {:?}", r); }

    // test each page
    for pl in page_lines {
        println!("Page List: {pl}");
        let pagelist: Vec<i32> = pl.split(",").map(|x| {
            x.parse::<i32>().expect("aprse")
        }).collect();



    
        let page_list_ok = test_page_list(pagelist.clone(), &rules);
        println!("page list good={page_list_ok} ::: {pl}");
        if page_list_ok { 
            let midpage = get_middle_page(pagelist.clone());
            println!("   midpage {midpage}");
            correct_row_count += midpage;
         } else {
            let midpage = fix_pagelist_and_return_middle(pagelist.clone(),&rules);
            println!("   CORRECTED midpage {midpage}");
            corrected_page_sums += midpage;
         };
    }
    


    println!("PART2: ***** corrected page sums is {corrected_page_sums}");

    return correct_row_count as u64;

}