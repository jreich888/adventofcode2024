// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashSet;


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


fn get_elev_at(full_fs: &mut Vec<Vec<i32>>, x:i32, y:i32, dims:(i32,i32)) -> i32 {
    // either get elev at coord, or -1 if off map
    let row =full_fs.get(y as usize);
    if row == None { return -1; }
    let cell = row.unwrap().get(x as usize);
    if cell == None { return -1; }
    return *cell.unwrap();
    
}



fn count_trail_inner(full_fs: &mut Vec<Vec<i32>>, x:i32, y:i32, dims:(i32,i32), elev:i32) -> (i32,HashSet<(i32,i32)>){

    let mut peak_set: HashSet<(i32,i32)> = HashSet::new(); 

    // did we reach the top of the trail, count it!
    if elev == 9 {
        peak_set.insert((x,y));
        return (1,peak_set);
    }

    let mut peak_set: HashSet<(i32,i32)> = HashSet::new(); 
    let mut trail_count = 0;


    // check four sides
    let left = get_elev_at(full_fs, x-1, y, dims);
    if left == elev+1 {
        // println!("found elev {left} pos at {},{y}",x-1);
        let nps = count_trail_inner(full_fs, x-1, y, dims, elev+1);
        peak_set.extend(nps.1);
        trail_count += nps.0;
    }
    let right = get_elev_at(full_fs, x+1, y, dims);
    if right == elev+1 {
        // println!("found elev {right} pos at {},{y}",x+1);
        let nps = count_trail_inner(full_fs, x+1, y, dims, elev+1);
        peak_set.extend(nps.1);
        trail_count += nps.0;
    }
    let up = get_elev_at(full_fs, x, y-1, dims);
    if up == elev+1 {
        // println!("found elev {up} pos at {},{}",x,y-1);
        let nps = count_trail_inner(full_fs, x, y-1, dims, elev+1);
        peak_set.extend(nps.1);
        trail_count += nps.0;
    }
    let down = get_elev_at(full_fs, x, y+1, dims);
    if down == elev+1 {
        // println!("found elev {down} pos at {},{}",x,y+1);
        let nps = count_trail_inner(full_fs, x, y+1, dims, down);
        peak_set.extend(nps.1);
        trail_count += nps.0;
    }

    return (trail_count,peak_set);
}


fn count_trails(full_fs: &mut Vec<Vec<i32>>, x:i32, y:i32, dims:(i32,i32)) -> (i32,i32) {
    let elev = 0;
    let peak_set = count_trail_inner(full_fs, x, y, dims, elev);
    println!("TRAILHEAD at {x},{x} has trail_count {} peak_count {} peak_set {:?}.", peak_set.0, peak_set.1.len(), peak_set.1);
    return (peak_set.0, peak_set.1.len() as i32);
    
}


pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut topo_map: Vec<Vec<i32>> = Vec::new();
    let mut count_p1 = 0;
    let mut count_p2 = 0;

    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers
    for l in lines {
        let row = l.chars().map( |c| {
            c.to_digit(10).expect("parse") as i32
            
        }).collect();
        topo_map.push(row);
    }

    for r in topo_map.clone() {
        println!("{:?}", r);
    }

    let dims: (i32, i32) = (topo_map[0].len() as i32, topo_map.len() as i32);

    // iterate looking for trailheads
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            if (topo_map[y as usize][x as usize] == 0) {
                // println!("found trailhead at {x},{y}");
                let trail_count = count_trails(&mut topo_map, x, y, dims);
                println!("found trailhead at {x},{y} trails={} {}", trail_count.0, trail_count.1);
                count_p1 += trail_count.1;
                count_p2 += trail_count.0;
            }
        }
    }

    println!("Final counts are {count_p1} {count_p2}");

    return count_p2 as u64;

}