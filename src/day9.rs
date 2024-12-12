// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;


fn display_fs(mut full_fs: &Vec<i32>) -> String {
    let mut s = String::new();
    for i in full_fs.iter() {
        if *i<0 { s = s + ".";}
        else { 
            let s2 =  format!("{i}");
            s = s + s2.as_str();
        }
        
    }
        // let result: &str= full_fs.iter().map(|i| {
        //     // let mut c = '.';
        //     if *i >= 0 {char::from_u32(*i as u32).expect("expect")}
        //     else {'.'}
        //     // c = ' ';
        //     // 8;
        // }).collect();

    return s;
}


fn calc_chksum(full_fs: &Vec<i32>) -> u64 {
    let mut sum = 0u64;
    let mut  pos = 0u64;
    for i in full_fs.iter() {
        if *i >= 0 {
            sum += pos * *i as u64;
        }
        pos += 1;
    }

    return sum;        

}

fn compact_fs(full_fs: &mut Vec<i32>) {
    let mut front_idx: usize = 0;
    let mut end_idx = full_fs.len()-1;

    loop {
        // advance front_idx to first space
        while full_fs[front_idx] != -1 && front_idx < end_idx {
            front_idx += 1;
        }

        if front_idx == end_idx {
            // println!("end of loop 1, breaking out");
            break;
        }

        while full_fs[end_idx] == -1 && front_idx < end_idx {
            end_idx -=  1;
        }
        if front_idx == end_idx {
            // println!("end of loop 2, breaking out");
            break;
        }

        let n = full_fs[end_idx];
        // println!("Moving {n} from {end_idx} to {front_idx}", );
        full_fs[front_idx] = n;
        full_fs[end_idx] = -1;

        // println!( "full_fs {}", display_fs(full_fs) );


    }



    
}

fn compact_fs_p2(full_fs: &mut Vec<i32>) {
    let mut front_idx: usize = 0;
    let mut end_idx = full_fs.len()-1;

    // last file should be at end of file here
    let mut last_filenum = full_fs[end_idx];

    while last_filenum >= 0 {

        // find pos and length of last filenum
        let start = full_fs.iter().position(|n| *n == last_filenum).unwrap();
        let num = full_fs.iter().filter(|n| **n == last_filenum).count();

        // println!("considering fn {last_filenum} count {num} at {start}");

        // find the first free space of at least NUM spaces, and move the file there
        let mut open_pos = 0 as usize;
        let mut open_count = 0 as usize;
        while open_pos < start {
            if full_fs[open_pos] == -1 {
                // count size
                open_count = 1;
                let i = 0 as usize;
                while full_fs[open_pos+open_count] == -1 {
                    open_count += 1;
                }

                // println!("found open count {open_count} at {open_pos}");
                if open_count >= num {
                    // println!("WORKS");
                    break;
                } else {
                    open_pos += open_count;
                }
            }
            open_pos += 1;
            open_count = 0;
        }

        if open_count >= num {
            // println!("moving fn {last_filenum} count {num} from {start} to {open_pos}");
            for i in 0..num {
                full_fs[open_pos+i] = last_filenum;
                full_fs[start+i] = -1;
            }
        }

        // println!( "full_fs {}", display_fs(full_fs) );


        last_filenum -= 1;
    }



    
}


pub fn process_lines(lines:Vec<String>) -> u64 {

    let source_fs_layout = &lines[0];
    let mut full_fs: Vec<i32> = Vec::new();

    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers
    let mut fssize: i32 = 0;
    let mut file_num = 0;
    let mut is_file = true;
    for c in source_fs_layout.as_str().chars() {
        let cnum = c.to_digit(10).expect("parse char");
        if is_file {
            // println!("file: {file_num} len={c} {cnum}");
            full_fs.extend( vec![file_num; cnum as usize].iter());
            file_num += 1;
        } else {
            // println!("space:  len={c} {cnum}");
            full_fs.extend( vec![-1 as i32; cnum as usize].iter());
        }
        fssize += cnum as i32;
        is_file = !is_file;
    }

    // println!("fs size {fssize}");
    // println!("full FS: {:?}", full_fs);
    // println!("full FS: {}", display_fs(&mut full_fs));
    // compact_fs(&mut full_fs);
    compact_fs_p2(&mut full_fs);
    // println!("full FS: {}", display_fs(&mut full_fs));
    // println!("full FS: {}", full_fs.join(" "));

    // create a new string of size of files above
    // let full_fs = Box::new([0i32; fssize]);
    // 

    // 0099811188827773336446555566..............
    // 0099811188827773336446555566..............

    // 00992111777244.333344.5555566666777.888889
    // 00992111777.44.333....5555.6666.....8888..

    let chksum = calc_chksum(&mut full_fs);



    return chksum as u64;

}