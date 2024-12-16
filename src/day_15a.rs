// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashSet;

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::coord;
// use multimap::MultiMap;

// Attempt 2 of P2, doing some rewrites


pub fn score_map( map: &Vec<Vec<char>> ) ->u64 {
    let mut score = 0u64;
    for y in 0..map.len()  {
        let r = &map[y];
        for x in 0..r.len() {
            let c = r[x];
            if c=='O' || c=='[' { score += 100*(y as u64) + x as u64; }
        }
    }
    return score;
}


pub fn score_map_wall( map: &Vec<Vec<char>> ) ->u64 {
    let mut score = 0u64;
    for y in 0..map.len()  {
        let r = &map[y];
        for x in 0..r.len() {
            let c = r[x];
            if c=='#' || c=='#' { score += 100*(y as u64) + x as u64; }
        }
    }
    return score;
}


pub fn print_map( map: &Vec<Vec<char>> ) {
    for r in map.clone() {
        println!("{}", r.iter().collect::<String>());
    }
}
pub fn scale_map( map: &Vec<Vec<char>> ) -> Vec<Vec<char>> {
    let mut map2 : Vec<Vec<char>> = Vec::new();
    for r in map.clone() {
        let mut r2 : Vec<char> = Vec::new();
        r.iter().for_each( |c| {
            let c2 = match c {
                '@' => ['@','.'],
                '#' => ['#','#'],
                'O' => ['[',']'],
                _ => ['.','.'],
            };
            // println!("cc {:?}",c2);
            r2.extend(c2);
        });
        // println!("row {:?}",r2);
        map2.push(r2);
    }


    print_map(&map2);

    return map2;
}



fn is_box(map: &mut Vec<Vec<char>>, loc: &geo::Coord<i32>) -> bool  {
    let c = get_loc(map,loc);
    return c=='[' || c==']';
}


fn get_loc(map: &mut Vec<Vec<char>>, loc: &geo::Coord<i32>) -> char  {
    let r = map.get(loc.y as usize);
    if r == None { return '#'; };
    return *r.unwrap().get(loc.x as usize).unwrap_or(&'#');
}
// fn move_robot(map: &mut Vec<Vec<char>>, robot_pos: &mut geo::Coord<i32>, movetype: char) -> bool {
//     let movedelta = match movetype {
//         '<' => geo::coord!{x:-1,y:0},
//         '^' => geo::coord!{x:0,y:-1},
//         '>' => geo::coord!{x:1,y:0},
//         'v' => geo::coord!{x:0,y:1},
//         _ => geo::coord!{x:0,y:1},
//     };

//     let next_pos = *robot_pos+movedelta;

//     // can we move?  if there is a space before we hit the wall in a direction.
//     let mut found_space = false;
//     let mut space_pos = next_pos.clone();
//     loop {
//         let c = get_loc(map, &space_pos);
//         if c=='.' {
//             found_space = true;
//             break;
//         }
//         if c=='#' { break; }
//         space_pos = space_pos+movedelta;
//     }

//     // no space before wall, so cannot move at all
//     if !found_space { return false; }

//     // if we didn't return, we'll remember the space pos from above

//     let next_c = get_loc(map, &next_pos);
//     if next_c=='O' {
//         // move the box to the previous found space pos
//         map[space_pos.y as usize][space_pos.x as usize] = 'O';
//     }
//     // robot always moves to next spot and leaves a space behind
//     map[next_pos.y as usize][next_pos.x as usize] = '@';
//     map[robot_pos.y as usize][robot_pos.x as usize] = '.';

//     *robot_pos = next_pos;

//     return true;
// }

// const LEFT_BOX_DELTA: geo::Coord<i32> = coord!{x:-1,y:0};
const RIGHT_SIDE_BOX_DELTA: geo::Coord<i32> = coord!{x:1,y:0};


fn do_p2_box_move(map: &mut Vec<Vec<char>>, box_pos: &mut geo::Coord<i32>, movedelta: &geo::Coord<i32>)  {
    // Just does the move, does not do any testing
    let dest_pos = *box_pos + *movedelta;

    map[box_pos.y as usize][box_pos.x as usize] = '.';
    map[box_pos.y as usize][1+box_pos.x as usize] = '.';
    map[dest_pos.y as usize][dest_pos.x as usize] = '[';
    map[dest_pos.y as usize][1+dest_pos.x as usize] = ']';
}


// fn test_move_box_vert_p2( map: &mut Vec<Vec<char>>, 
//     box_pos: &mut geo::Coord<i32>, // must be left coord of box
//     movetype: char,
//     movedelta: &geo::Coord<i32>,
//     vert_move: bool ) -> bool 
// {
//     // getting trickier
//     // check pos after box
//     let mut test_pos = *box_pos + *movedelta;
//     let mut test_pos2 = test_pos + RIGHT_SIDE_BOX_DELTA;

//     let c1 = get_loc(map,&test_pos);
//     let c2 = get_loc(map,&test_pos2);
//     // println!("test_move_box_vert_p2 bp={:?} {c1} {c2}", box_pos);

//     // check left side
//     let mut can_move_left = false;
//     if c1 == '[' {
//         can_move_left = test_move_box_vert_p2(map, &mut test_pos, movetype, movedelta, vert_move);
//     } else if c1 == ']' {
//         let mut bp = test_pos + LEFT_BOX_DELTA;
//         can_move_left = test_move_box_vert_p2(map, &mut bp, movetype, movedelta, vert_move);
//     } else if c1 == '.' { can_move_left=true; }
//     // check right side
//     let mut can_move_right = false;
//     if c2=='[' {
//         // only proceed if that box moves
//         can_move_right = move_box_p2(map, &mut test_pos2, movetype, movedelta, vert_move);
//     } else if c2 == ']' { can_move_right=true; // processed above
//     } else if c2=='.' { can_move_right=true;}

//     println!("test_move_box_vert_p2 bp={:?} {c1} {c2} {can_move_left} {can_move_right}", box_pos);

//     return can_move_left && can_move_right;
// }


fn move_box_p2( map: &mut Vec<Vec<char>>, 
    init_box_pos: &mut geo::Coord<i32>, // must be left coord
    movetype: char,
    movedelta: &geo::Coord<i32>,
    vert_move: bool,
    test_only: bool ) -> bool 
{
    assert!(is_box(map,init_box_pos));    
    let mut box_pos = *init_box_pos;
    let c = get_loc(map,&box_pos);
    if c == ']' { box_pos.x = box_pos.x - 1; }
    let c = get_loc(map,&box_pos);
    assert_eq!(c, '[');

    let mut can_move = false;
    // let mut boxes_to_move = HashSet::new();

    //////////// RIGHT 
    if movetype=='>' {
        // most basic
        // check pos after box; to right it is two moves
        let mut test_pos = box_pos + *movedelta + *movedelta;
        let c = get_loc(map,&test_pos);
        // is there a box there
        if c=='.' { can_move=true; }
        else if is_box(map, &test_pos) {
            can_move = move_box_p2(map, &mut test_pos, movetype, movedelta, vert_move, test_only);
        }
    }
    //////////// LEFT 
    if movetype=='<' {
        // more basic
        // check pos after box
        let mut test_pos = box_pos + *movedelta;
        // is there a box there
        let c = get_loc(map,&test_pos);
        if c=='.' { can_move=true; }
        else if is_box(map, &test_pos) {
            can_move = move_box_p2(map, &mut test_pos, movetype, movedelta, vert_move, test_only);
        }
    }
    //////////// UP or DOWN
    if movetype=='^' || movetype=='v'{
        // getting tricker
        // need to test ALL affected boxes before we start to actually move any

        // this is bad?  why?
        // let can_move = test_move_box_vert_p2(map, box_pos, movetype, movedelta, vert_move);
        // if !can_move { return can_move; }


        // move any boxes in our way
        let mut test_pos1 = box_pos + *movedelta;
        let mut test_pos2 = box_pos + *movedelta + RIGHT_SIDE_BOX_DELTA;
        // is there a box there
        let c1 = get_loc(map,&test_pos1);
        let c2 = get_loc(map,&test_pos2);
        let mut can_move_1 = false;
        let mut can_move_2 = false;

        if c1 == '.' { can_move_1=true; }
        if c1 == '[' || c1 == ']' {
            can_move_1 = move_box_p2(map, &mut test_pos1, movetype, movedelta, vert_move, test_only);
        } 
        if c2 == '.' || c2 == ']' { can_move_2=true; }
        if c2 == '['  {  // only need to check if start of new box
            can_move_2 = move_box_p2(map, &mut test_pos2, movetype, movedelta, vert_move, test_only);
        } 
        can_move = can_move_1 && can_move_2;

        // println!("move_box_p2 up/down bp={:?} {c1} {c2} {can_move_1} {can_move_2}", box_pos);


    }
    if can_move && !test_only {
        do_p2_box_move(map, &mut box_pos, movedelta);
    }
    return can_move;
}




// fn can_move_p2(map: &mut Vec<Vec<char>>, 
//     robot_pos: &mut geo::Coord<i32>, 
//     movedelta: &geo::Coord<i32>,
//     vert_move: bool) -> bool {

//     let mut found_space = false;
//     let mut temp_pos = robot_pos.clone();

//     // if next is space, always good
//     if get_loc(map, &temp_pos) == '.' { return  true; }
//     if !vert_move {
//     // look for space in this row, just like p1
//         let mut found_space = false;
//         loop {
//             let c = get_loc(map, &temp_pos);
//             if c=='.' {
//                 found_space = true;
//                 break;
//             }
//             if c=='#' { break; }
//             temp_pos = temp_pos+movedelta;
//         }
//         return found_space;
//     }
//     // vertical test is more complex 
    
//     if !vert_move { return found_space; }

//     // for vertical, need to 
// }



fn move_robot_p2(map: &mut Vec<Vec<char>>, robot_pos: &mut geo::Coord<i32>, movetype: char) -> bool {
    let movedelta = match movetype {
        '<' => geo::coord!{x:-1,y:0},
        '^' => geo::coord!{x:0,y:-1},
        '>' => geo::coord!{x:1,y:0},
        'v' => geo::coord!{x:0,y:1},
        _ => None.expect("failed"),
    };
    let vert_move = movetype=='^' || movetype=='v';

    let mut next_pos = *robot_pos+movedelta;

    // can we move?  if there is a space before we hit the wall in a direction.
    let c = get_loc(map, &next_pos);
    let mut can_move = false;
    if c=='.' { can_move = true; }
    if c=='[' || c==']' { 
        // move box will figure out if it can move and move it if it (and all behind) can
        // will also adjust to actual box coord if necessary
        can_move = move_box_p2(map, &mut next_pos, movetype, &movedelta, vert_move, true); 

        if can_move { 
            move_box_p2(map, &mut next_pos, movetype, &movedelta, vert_move, false); 
        }
    }

    // no space before wall, so cannot move at all
    if !can_move { return false; }

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

    let mut map_p2  = scale_map(&mut map);

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

    if false {
    for m in steps.clone() {
        let moved = move_robot_p2(&mut map,&mut robot_pos,m);
        println!("move {m} {moved}");
        // print_map(&map);
    }
    }

    let mut score = score_map(&map);

    //////// start of P2
    /// 
    let wall_score_start = score_map_wall(&map);
    
    // Find the robot position in p2
    for y in 0..map_p2.len() {
        if map_p2[y].contains(&'@') {
            let x = map_p2[y].iter().position(|&r| r == '@').unwrap();
            robot_pos.x = x as i32;
            robot_pos.y = y as i32;
        }
    }
    println!("Robot pos {:?}", robot_pos);

    for m in steps.clone() {
        let moved = move_robot_p2(&mut map_p2,&mut robot_pos,m);
        println!("move {m} {moved}");
        // print_map(&map_p2);
    }

    println!("FINAL");
    print_map(&map_p2);

    score = score_map(&map_p2);
    let wall_score_end = score_map_wall(&map);

    println!("wall scores {wall_score_start} {wall_score_end}");

    return score as u64;
}