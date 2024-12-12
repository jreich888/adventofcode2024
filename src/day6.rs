// use regex::{bytes, Regex};
use std::collections::HashSet;


// fn p1_count_steps(map:&Vec<Vec<char>>,guard_start:(i32,i32),guard_start_dir:(i32,i32)) -> i32 {
    
//     let mut pos_history: HashSet<i64> = HashSet::new();
//     let mut guard_pos = (guard_start.0,guard_start.1);
//     let mut guard_dir = (guard_start_dir.0,guard_start_dir.1);
//     let map_size = (map.get(0).unwrap().len() as i32, map.len() as i32);
//     let step_count = 0;

//     loop {
//         // what's at next pos
//         let next_coord = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
//         // println!("next_coord: {:?}", next_coord);
//         if (next_coord.0 < 0) || (next_coord.0 > map_size.0-1)  ||
//             (next_coord.1 < 0) || (next_coord.1 > map_size.1-1) {
//                 println!("end of map, steps is next_coord: {:?} {step_count}", next_coord);
//                 break;
//         }


        

//         let maprow = &mut map[next_coord.1 as usize]; // .clone().get(next_coord.1 as usize).unwrap();
//         let prev_val: char = maprow[next_coord.0 as usize];
//         println!("value was {prev_val} at {:?}", next_coord);

//         // do we turn?
//         if prev_val == POS_OBSTACLE || prev_val == POS_OBSTACLE_2 {
//             match guard_dir {
//                 (0,-1) => guard_dir = (1,0),
//                 (1,0) => guard_dir = (0,1),
//                 (0,1) => guard_dir = (-1,0),
//                 (-1,0) => guard_dir = (0,-1),
//                 _ => println!("ERROR: BAD DIRECTION")
//             }
//             println!( "obstacle!  turning right new dir {:?}", guard_dir);
//             continue;
//         }



//         maprow[next_coord.0 as usize] = POS_STEPPED;

//         // let got = std::mem::replace(&mut maprow[next_coord.0 as usize], POS_STEPPED);
//         // println!("got was {got}");
//         // std::mem::replace(&mut maprow[next_coord.0], POS_STEPPED)

//         if prev_val != POS_STEPPED {
//             println!( "new step {step_count}");
//             step_count += 1;
//         } else {
//             println!( "repeated step");
//         }

//         guard_pos = next_coord;


//         // encode our position into a long int; assumes our map is less than 256*256
//         let encode_position: i64 = guard_pos.0 as i64 + guard_pos.1  as i64 * 256 + 
//             guard_dir.0 as i64*256*256 + guard_dir.1 as i64*256*256*256;
        
//         if pos_history.contains(&encode_position) {
//             println!( "WE ARE LOOPING");
//             break;
//         }
//         pos_history.insert(encode_position);



//     }

    
//     for mr in map.clone() {
//         let s: String = mr.into_iter().collect();
//         println!("map row: {}", s);
//     }
//     println!("guard_pos: {:?}", guard_pos);
//     println!("guard_dir: {:?}", guard_dir);
//     println!("map_size: {:?}", map_size);



//     return step_count as u64;

// }

pub fn test_loop(map:Vec<Vec<char>>,guard_start:(i32,i32),guard_start_dir:(i32,i32)) -> bool {
    
    let mut pos_history: HashSet<i64> = HashSet::new();
    let mut guard_pos = (guard_start.0,guard_start.1);
    let mut guard_dir = (guard_start_dir.0,guard_start_dir.1);
     let map_size = (map.get(0).unwrap().len() as i32, map.len() as i32);

    loop {
        // what's at next pos
        let next_coord = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
        // println!("next_coord: {:?}", next_coord);
        if (next_coord.0 < 0) || (next_coord.0 > map_size.0-1)  ||
            (next_coord.1 < 0) || (next_coord.1 > map_size.1-1) {
                println!("end of map, not a loop ");
                return false;
        }


        let maprow = &map[next_coord.1 as usize]; // .clone().get(next_coord.1 as usize).unwrap();
        let prev_val: char = maprow[next_coord.0 as usize];
        // println!("value was {prev_val} at {:?}", next_coord);

        // do we turn?
        if prev_val == POS_OBSTACLE || prev_val == POS_OBSTACLE_2 {
            match guard_dir {
                (0,-1) => guard_dir = (1,0),
                (1,0) => guard_dir = (0,1),
                (0,1) => guard_dir = (-1,0),
                (-1,0) => guard_dir = (0,-1),
                _ => println!("ERROR: BAD DIRECTION")
            }
            // println!( "obstacle!  turning right new dir {:?}", guard_dir);
            continue;
        }



        // maprow[next_coord.0 as usize] = POS_STEPPED;

        // let got = std::mem::replace(&mut maprow[next_coord.0 as usize], POS_STEPPED);
        // println!("got was {got}");
        // std::mem::replace(&mut maprow[next_coord.0], POS_STEPPED)

        // if prev_val != POS_STEPPED {
        //     println!( "new step {step_count}");
        //     step_count += 1;
        // } else {
        //     println!( "repeated step");
        // }

        guard_pos = next_coord;


        // encode our position into a long int; assumes our map is less than 256*256
        let encode_position: i64 = guard_pos.0 as i64 + guard_pos.1  as i64 * 256 + 
            guard_dir.0 as i64*256*256 + guard_dir.1 as i64*256*256*256;
        
        if pos_history.contains(&encode_position) {
            // println!( "WE ARE LOOPING");
            return true;
        }
        pos_history.insert(encode_position);

    }

}



const POS_EMPTY: char = '.';
const POS_OBSTACLE: char = '#';
const POS_OBSTACLE_2: char = 'O';
const POS_STEPPED: char = 'x';


pub fn process_lines(lines:Vec<String>) -> u64 {

    // Lets process the map.  Input: guard is arrow (any 4 dirs), and obstacles are #
    // Convert input to Vec<Vec<i32>> with: const values above

    let mut map : Vec<Vec<char>> = Vec::new();
    let mut guard_pos : (i32,i32) = (-1,-1);
    let mut guard_dir : (i32,i32) = (-1,-1);
    // let mut step_count : i32 = 0;       // starts with guard in 1 place

    for l in lines {
        println!("row: {l}");
        let mut maprow : Vec<char> = Vec::new();
        // Parse line in one char at a time
        for c in l.as_str().chars() {
            // println!("  c: {c}");
            
            match c {
                '.' => maprow.push(POS_EMPTY),
                '#' => maprow.push(POS_OBSTACLE),
                _ => {
                    println!( "found guard {c} at pos {} {}", maprow.len(), map.len() );
                    guard_pos = (maprow.len() as i32, map.len() as i32);
                    guard_dir = (0,-1);
                    maprow.push(POS_STEPPED);
                    // step_count += 1;
                }
            }
        }
        map.push(maprow);
    }
    let map_size = (map.get(0).unwrap().len() as i32, map.len() as i32);

    for mr in map.clone() {
        let s: String = mr.into_iter().collect();
        println!("map row: {}", s);
    }
    println!("guard_pos: {:?}", guard_pos);
    println!("guard_dir: {:?}", guard_dir);
    println!("map_size: {:?}", map_size);

    let init_guard_pos = guard_pos.clone();
    let init_guard_dir = guard_dir.clone();

    let mut loop_count = 0;



    for y in 0..map_size.1 {
        for x in 0..map_size.0 {
            // let mutmap = &mut map;
            let mut before = POS_EMPTY;
            {
            let maprow = &mut map[y as usize];
            before = maprow[x as usize];
            }

            if before == POS_EMPTY {

{
    let maprow = &mut map[y as usize];
    // let s1: String = maprow.iter_mut().into_iter().map(|c| *c).collect();
    //                         println!("map row: {}", s1);

                    maprow[x as usize] = POS_OBSTACLE_2;
                    // let s2: String = maprow.iter_mut().into_iter().map(|c| *c).collect();
                    // println!("map row: {}", s2);
}

                let tl = test_loop(map.clone(), guard_pos, guard_dir);



                if tl {
                    loop_count += 1;
                    // println!("found a loop obstacle pos at {x},{y} count {loop_count}");
                } else {
                    // println!("not a loop with obstacle at {x},{y} count {loop_count}");
                }
                {
                let maprow = &mut map[y as usize];
                maprow[x as usize] = POS_EMPTY;
                }
            }
        }
    }

    // for y in 0..map_size.1 {
    //     for x in 0..map_size.0 {
    //         // let mutmap = &mut map;
    //         let maprow = &mut map.clone()[y as usize];
    //         let before = maprow[x as usize];

    //         if before == POS_EMPTY {
    //             maprow[x as usize] = POS_OBSTACLE_2;
    //             let tl = test_loop(&mut map, guard_pos, guard_dir);
    //             if tl {
    //                 loop_count += 1;
    //                 println!("found a loop obstacle pos at {x},{y} count {loop_count}");
    //             } else {
    //                 println!("not a loop with obstacle at {x},{y} count {loop_count}");
    //             }
    //             maprow[x as usize] = POS_EMPTY;
    //         }
    //     }
    // }

    return loop_count;

    // // Add a loop obstacle at test position
    // {
    //     let maprow = &mut map[6];
    //     maprow[3] = POS_OBSTACLE_2;
    // }

    // let tl = test_loop(&map, guard_pos, guard_dir);
    // println!("test loop returned {tl}");
    // return 0;

    // let mut pos_history: HashSet<i64> = HashSet::new();

    // loop {
    //     // what's at next pos
    //     let next_coord = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
    //     // println!("next_coord: {:?}", next_coord);
    //     if (next_coord.0 < 0) || (next_coord.0 > map_size.0-1)  ||
    //         (next_coord.1 < 0) || (next_coord.1 > map_size.1-1) {
    //             println!("end of map, steps is next_coord: {:?} {step_count}", next_coord);
    //             break;
    //     }


        

    //     let maprow = &mut map[next_coord.1 as usize]; // .clone().get(next_coord.1 as usize).unwrap();
    //     let prev_val: char = maprow[next_coord.0 as usize];
    //     println!("value was {prev_val} at {:?}", next_coord);

    //     // do we turn?
    //     if prev_val == POS_OBSTACLE || prev_val == POS_OBSTACLE_2 {
    //         match guard_dir {
    //             (0,-1) => guard_dir = (1,0),
    //             (1,0) => guard_dir = (0,1),
    //             (0,1) => guard_dir = (-1,0),
    //             (-1,0) => guard_dir = (0,-1),
    //             _ => println!("ERROR: BAD DIRECTION")
    //         }
    //         println!( "obstacle!  turning right new dir {:?}", guard_dir);
    //         continue;
    //     }



    //     maprow[next_coord.0 as usize] = POS_STEPPED;

    //     // let got = std::mem::replace(&mut maprow[next_coord.0 as usize], POS_STEPPED);
    //     // println!("got was {got}");
    //     // std::mem::replace(&mut maprow[next_coord.0], POS_STEPPED)

    //     if prev_val != POS_STEPPED {
    //         println!( "new step {step_count}");
    //         step_count += 1;
    //     } else {
    //         println!( "repeated step");
    //     }

    //     guard_pos = next_coord;


    //     // encode our position into a long int; assumes our map is less than 256*256
    //     let encode_position: i64 = guard_pos.0 as i64 + guard_pos.1  as i64 * 256 + 
    //         guard_dir.0 as i64*256*256 + guard_dir.1 as i64*256*256*256;
        
    //     if pos_history.contains(&encode_position) {
    //         println!( "WE ARE LOOPING");
    //         break;
    //     }
    //     pos_history.insert(encode_position);



    // }

    
    // for mr in map.clone() {
    //     let s: String = mr.into_iter().collect();
    //     println!("map row: {}", s);
    // }
    // println!("guard_pos: {:?}", guard_pos);
    // println!("guard_dir: {:?}", guard_dir);
    // println!("map_size: {:?}", map_size);



    // return step_count as u64;

}