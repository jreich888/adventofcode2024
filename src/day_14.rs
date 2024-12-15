// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashSet;
use std::collections::HashMap;
use geo::coord;
use multimap::MultiMap;


#[derive(Debug,Clone)]
struct Robot {
    pos: geo::Coord<i32>,
    vel: geo::Coord<i32>,
}



fn test_filled_row(robots : &Vec<Robot>, size: &geo::Coord<i32>) -> bool {
    let threshold = 10;
    let mut consecutive = 0;

    let mut coordcount : HashMap<geo::Coord<i32>,i32> = HashMap::new();

    for r in robots.clone() {
        let n = coordcount.remove(&r.pos).unwrap_or(0);
        coordcount.insert(r.pos, n+1);
    }

    for y in 0..size.y {
        consecutive = 0;
        for x in  0..size.x {
            let n = coordcount.get(&coord!(x:x,y:y)).unwrap_or(&0);
            if n > &0 { consecutive += 1 } else { consecutive=0; }
            if consecutive > threshold { return true }
        }
    }
    return false;

}


fn count_quadrants(robots : &Vec<Robot>, size: &geo::Coord<i32>) -> [i32;4] {
    let mut counts = [0,0,0,0];
    let mid_x = size.x/2; // sizes are odd, so midline is exactly this rounde
    let mid_y = size.y/2;
    for r in robots {
        if r.pos.x<mid_x && r.pos.y<mid_y { counts[0] += 1 }
        if r.pos.x>mid_x && r.pos.y<mid_y { counts[1] += 1 }
        if r.pos.x<mid_x && r.pos.y>mid_y { counts[2] += 1 }
        if r.pos.x>mid_x && r.pos.y>mid_y { counts[3] += 1 }
    }
    return counts;
}

// fn step(robots : &Vec<Robot>, size: &geo::Coord<i32>) {
//     for r in robots {
//         let n= 1;
//         let nx = (r.pos.x + r.vel.x*n).rem_euclid(size.x);
//         let ny = (r.pos.y + r.vel.y*n).rem_euclid(size.y);
//         r.pos.x = nx;
//         r.pos.y = ny;

//     }
// }


fn print_map(robots : &Vec<Robot>, size: &geo::Coord<i32>) {
    let mut coordcount : HashMap<geo::Coord<i32>,i32> = HashMap::new();

    for r in robots.clone() {
        let n = coordcount.remove(&r.pos).unwrap_or(0);
        coordcount.insert(r.pos, n+1);
    }

    for y in 0..size.y {
        for x in  0..size.x {
            let n = coordcount.get(&coord!(x:x,y:y)).unwrap_or(&0);
            let mut s: String = ".".to_string();
            if *n>0 { s = n.to_string() };
            print!("{s}");
        }
        println!("");
    }
}

pub fn process_lines(lines:Vec<String>) -> u64 {
  

    

    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers


    let mut robots : Vec<Robot> = Vec::new();

    for l in lines {
        let f : Vec<&str> = l.split_ascii_whitespace().collect();
        let pos : Vec<i32> = f[0].replace("p=","").split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        let vel : Vec<i32> = f[1].replace("v=","").split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        robots.push( Robot { pos: coord!(x:pos[0], y:pos[1]), vel: coord!{x:vel[0], y:vel[1]} });
    }

    for r in robots.clone() {
        println!( "robot {:?}", r);
    }


    let p1_tile_size: geo::Coord<i32> = coord!(x:11,y:7);
    let p2_tile_size: geo::Coord<i32> = coord!(x:101,y:103);
    let tile_size = p2_tile_size;

    for x in -20..20 {
        // remainder test
        println!( "x%11 {x} {}", x%11);
    }
    for x in -20..20 {
        // remainder test
        let xi: i32 = x;
        println!( "x%11 {xi} {}", xi.rem_euclid(11));
    }

    println!("before");
    print_map(&robots, &tile_size);

    let mut robots2 : Vec<Robot> = Vec::new();

    let iters = 100;
    for r in robots.clone() {
        let nx = (r.pos.x + r.vel.x*iters).rem_euclid(tile_size.x);
        let ny = (r.pos.y + r.vel.y*iters).rem_euclid(tile_size.y);

        robots2.push( Robot { pos: coord!(x:nx, y:ny), vel: r.vel });


    }
    println!("after {iters} moves");
    print_map(&robots2, &tile_size);

    let quad_counts = count_quadrants(&robots2, &tile_size);
    println!("quad counts {:?}", quad_counts);
    let safety_factor = quad_counts[0]*quad_counts[1]*quad_counts[2]*quad_counts[3];

    let mut rtree = robots.clone();
    for n in 1..=10000 {
        for i in 0..rtree.len() {
            // for mut r in rtree.clone() {
            let nx = (rtree[i].pos.x + rtree[i].vel.x).rem_euclid(tile_size.x);
            let ny = (rtree[i].pos.y + rtree[i].vel.y).rem_euclid(tile_size.y);
            rtree[i].pos.x = nx;
            rtree[i].pos.y = ny;
        }

        let solid_row = test_filled_row(&rtree, &tile_size);
        if solid_row {
            // step(&rtree,&tile_size);
            println!("Step {n}");
            print_map(&rtree, &tile_size);
        }
    }


    return safety_factor as u64;
}