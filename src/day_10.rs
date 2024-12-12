// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::{collections::HashSet, ptr::addr_eq};
use geo::coord;



fn get_elev_at(full_fs: &mut Vec<Vec<i32>>, loc : geo::Coord<i32>) -> i32 {
    // either get elev at coord, or -1 if off map
    let row =full_fs.get(loc.y as usize);
    if row == None { return -1; }
    return * row.unwrap().get(loc.x as usize).unwrap_or(&-1);
}


fn count_trail_inner(full_fs: &mut Vec<Vec<i32>>, loc : geo::Coord<i32>, elev:i32) -> (i32,HashSet<geo::Coord<i32>>){

    let mut peak_set: HashSet<geo::Coord<i32>> = HashSet::new(); 

    // did we reach the top of the trail, count it!
    if elev == 9 {
        peak_set.insert(loc);
        return (1,peak_set);
    }

    let mut trail_count = 0;

    let dirs = [ 
        coord!{x:-1,y:0},
        coord!{x:1,y:0},
        coord!{x:0,y:-1},
        coord!{x:0,y:1}
    ];

    for dxy in dirs {
        let adjacent_loc = loc + dxy;
        let adj_elev = get_elev_at(full_fs, adjacent_loc);
        if adj_elev == elev+1 {
            // println!("found elev {left} pos at {},{y}",x-1);
            let results = count_trail_inner(full_fs, 
                adjacent_loc, adj_elev );
            peak_set.extend(results.1);
            trail_count += results.0;    
        }
    }

    return (trail_count,peak_set);
}


fn count_trails(full_fs: &mut Vec<Vec<i32>>, loc : geo::Coord<i32>) -> (i32,i32) {
    let elev = 0;
    let peak_set = count_trail_inner(full_fs, loc, elev);
    println!("TRAILHEAD at {},{} has trail_count {} peak_count {} peak_set {:?}.", 
        loc.x, loc.y, 
        peak_set.0, peak_set.1.len(), peak_set.1);
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
                let location = coord!{x:x,y:y};
                let trail_count = count_trails(&mut topo_map, location);
                println!("found trailhead at {:?} trails={} {}", location, trail_count.0, trail_count.1);
                count_p1 += trail_count.1;
                count_p2 += trail_count.0;
            }
        }
    }

    println!("Final counts are {count_p1} {count_p2}");

    return count_p2 as u64;

}