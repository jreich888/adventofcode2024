// use regex::{bytes, Regex};

fn getcharat(  lines:&Vec<String>,
    x:i32,
    y:i32,
    dimx:usize,
    dimy:usize) -> &str {

    let mut work = "".to_string();
    
    if (x < 0) || (y<0) || (x >= dimx as i32) || (y >= dimy as i32) { return "."; }
    // otherwise get char and append to string
    let row = lines.get(y as usize).expect("asedf");
    let xus: usize = x as usize;
    let found_c: &str = row.as_str().get(xus..xus+1).expect("aaa");
    // println!("found char: {}", found_c);
    return found_c;
}

fn get4letters(  lines:&Vec<String>,
    x:usize,
    y:usize,
    dx:i32,
    dy:i32,
    dimx:usize,
    dimy:usize) -> String {

    let mut work = "".to_string();
    
    for i  in 0..4 {
        let curx = (x as i32) + (i*dx as i32);
        let cury = (y as i32) + (i*dy as i32);
        // check for limits 
        if (curx < 0) || (cury<0) || (curx >= dimx as i32) || (cury >= dimy as i32) { return work; }
        // otherwise get char and append to string
        let row = lines.get(cury as usize).expect("asedf");
        let curxus: usize = curx as usize;
        let found_c = row.as_str().get(curxus..curxus+1).expect("aaa");
        // println!("found char: {}", found_c);
        work = work + found_c;
    }

    return work;

}

fn search_xmas_8ways(lines:&Vec<String>,x:usize,y:usize) -> u32 {
    let mut count = 0;
    // watch for limits
    let dimx = lines.get(0).unwrap().len();
    let dimy = lines.len();

    // let mut dx: i32 = 0;
    // let mut dy: i32 = 0;

    for dx in -1..2 {
        for dy in -1..2 {
            // get the string 
            let test4chars = get4letters(lines, x, y, dx, dy, dimx, dimy);
            let does_match = test4chars.eq("XMAS");
            println!("Considering string at {x},{y} dir {dx},{dy} string {test4chars} match {does_match}");
            if does_match { count += 1; }

        }
    }


    return count;
}

fn search_mas_in_x(lines:&Vec<String>,x:usize,y:usize) -> u32 {
    let mut count = 0;
    // watch for limits
    let dimx = lines.get(0).unwrap().len();
    let dimy = lines.len();

    // get chars in one dir
    let mut c1 = getcharat(lines, x as i32 - 1, y as i32 -1, dimx, dimy);
    let mut c2 = getcharat(lines, x as i32 + 1, y as i32 +1, dimx, dimy);

    let mut poss_ok_one = (c1.eq("M") && c2.eq("S")) || (c1.eq("S") && c2.eq("M"));
    if !poss_ok_one { return 0 }

    // get chars in other dir
    c1 = getcharat(lines, x as i32 - 1, y as i32 +1, dimx, dimy);
    c2 = getcharat(lines, x as i32 + 1, y as i32 -1, dimx, dimy);

    let mut poss_ok_two = (c1.eq("M") && c2.eq("S")) || (c1.eq("S") && c2.eq("M"));


    if !poss_ok_two { return 0; }

    return 1;

}

pub fn process_lines(lines:Vec<String>) -> u64 {
    // convert lines into 2d array

    // let mut rows = Vec::new();

    let mut xmascount = 0;


    // for l in lines {
    //     let row = l.as_bytes().clone();
    //     rows.push(l.as_bytes().clone());
    // }

    // start searching the rows for XMAS
    let dimx = lines.get(0).unwrap().len();
    let dimy = lines.len();

    for y in 0..dimy {
        let row = lines.get(y).unwrap().as_bytes();
        for x in 0..dimx {
            let c = row[x];

            // Part 1
            // if c == b'X' {
            //     println!("Found an X at {x},{y}");
            //     // search for XMAS from here
            //     let found = search_xmas_8ways(&lines, x, y);
            //     xmascount += found;
            // }
            // Part 2
            if c == b'A' {
                println!("Found an A at {x},{y}");
                // search for XMAS from here
                let found = search_mas_in_x(&lines, x, y);
                xmascount += found;
            }
        }
    }
    return xmascount as u64;

}