// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashSet;
use std::collections::HashMap;
use geo::coord;
use multimap::MultiMap;


pub fn score_map( map: &Vec<Vec<char>> ) ->u64 {
    let mut score = 0u64;
    for y in 0..map.len()  {
        let r = &map[y];
        for x in 0..r.len() {
            let c = r[x];
            if c=='O' { score += 100*(y as u64) + x as u64; }
        }
    }
    return score;
}

pub fn print_map( map: &Vec<Vec<char>> ) {
    for r in map.clone() {
        println!("{}", r.iter().collect::<String>());
    }
}



fn get_loc(map: &mut Vec<Vec<char>>, loc: &geo::Coord<i32>) -> char  {
    let r = map.get(loc.y as usize);
    if r == None { return '#'; };
    return *r.unwrap().get(loc.x as usize).unwrap_or(&'#');
}
fn move_robot(map: &mut Vec<Vec<char>>, robot_pos: &mut geo::Coord<i32>, movetype: char) -> bool {
    let movedelta = match movetype {
        '<' => geo::coord!{x:-1,y:0},
        '^' => geo::coord!{x:0,y:-1},
        '>' => geo::coord!{x:1,y:0},
        'v' => geo::coord!{x:0,y:1},
        _ => geo::coord!{x:0,y:1},
    };

    let mut next_pos = *robot_pos+movedelta;

    // can we move?  if there is a space before we hit the wall in a direction.
    let mut found_space = false;
    let mut space_pos = next_pos.clone();
    loop {
        let c = get_loc(map, &space_pos);
        if c=='.' {
            found_space = true;
            break;
        }
        if c=='#' { break; }
        space_pos = space_pos+movedelta;
    }

    // no space before wall, so cannot move at all
    if !found_space { return false; }

    // if we didn't return, we'll remember the space pos from above

    let next_c = get_loc(map, &next_pos);
    if next_c=='O' {
        // move the box to the previous found space pos
        map[space_pos.y as usize][space_pos.x as usize] = 'O';
    }
    // robot always moves to next spot and leaves a space behind
    map[next_pos.y as usize][next_pos.x as usize] = '@';
    map[robot_pos.y as usize][robot_pos.x as usize] = '.';

    *robot_pos = next_pos;

    return true;
}

pub fn process_lines(lines:Vec<String>) -> u64 {
  

    

    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers


    

    let mut map : Vec<Vec<char>> = Vec::new();
    let mut steps : Vec<char> = Vec::new();

    let mut parsing_map = true;

    for l in lines {
        if l.trim().is_empty() {
            parsing_map = false;
            continue;
        }
        let row = l.trim().chars().collect();
        if parsing_map {
            map.push(row);
        } else {
            steps.extend(row);
        }
    }

    print_map(&map);
    println!("Moves:");
    println!("{}", steps.clone().iter().collect::<String>());

    // find the robot location
    let mut robot_pos: geo::Coord<i32> = geo::coord!{x:0,y:0};
    for y in 0..map.len() {
        if map[y].contains(&'@') {
            let x = map[y].iter().position(|&r| r == '@').unwrap();
            robot_pos.x = x as i32;
            robot_pos.y = y as i32;
        }
    }
    println!("Robot pos {:?}", robot_pos);

    for m in steps {
        let moved = move_robot(&mut map,&mut robot_pos,m);
        println!("move {m} {moved}");
        // print_map(&map);
    }

    let score = score_map(&map);

    return score as u64;
}