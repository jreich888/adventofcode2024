// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use core::num;
use std::{collections::{vec_deque, HashMap, HashSet, VecDeque, BTreeMap }, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
use lazy_static::lazy_static;
// use multimap::MultiMap;

////////////////////////////////////
/////////  NUM KEYPAD //////////////
////////////////////////////////////

fn num_keypad_char_to_pos(from: char) -> geo::Coord<i32> {
    let y = match from {
        '7'|'8'|'9' => 0,
        '4'|'5'|'6' => 1,
        '1'|'2'|'3' => 2,
        _ => 3
    };
    let x = match from {
        '7'|'4'|'1' => 0,
        '8'|'5'|'2'|'0' => 1,
        _ => 2
    };
    return coord!{x:x,y:y};
}

// return multiple paths if both left and right, and both are legal.
// otherwise return one path
fn num_keypad_move(from: char, to: char) -> Vec<String> {
    let start = num_keypad_char_to_pos(from);
    let end = num_keypad_char_to_pos(to);
    // let mut command = String::new().to_owned();

    let diff = end-start;

    // let vsteps = match diff.y {
    //     -3..-1 => String::new() + "^".repeat(diff.y.abs() as usize).as_str(),
    //     1..3 => String::new() +  "^".repeat(diff.y as usize).as_str(),
    //     _ => "".to_string()
    // };
    // let hsteps = match diff.x {
    //     -3..-1 => String::new() + "<".repeat(diff.x.abs() as usize).as_str(),
    //     1..3 => String::new() + ">".repeat(diff.x as usize).as_str(),
    //     _ => "".to_string()
    // };
    // let mut hsteps = "";
    let mut hsteps = (if diff.x<0 {"<"} else {">"}).repeat(diff.x.abs() as usize);
    let mut vsteps = (if diff.y<0 {"^"} else {"v"}).repeat(diff.y.abs() as usize);
    // if diff
    // let hsteps = "<".repeat(diff.x.abs() as usize);
    // let hsteps = match diff.x {
    //     -3..-1 => String::new() + "<".repeat(diff.x.abs() as usize).as_str(),
    //     1..3 => String::new() + ">".repeat(diff.x as usize).as_str(),
    //     _ => "".to_string()
    // };
    // println!("slns: {from} {to} H{hsteps}  V{vsteps}  {:?}",diff);
    let sln1 = vsteps.to_string() + hsteps.as_str() + "A";
    let sln2 = hsteps.to_string() + vsteps.as_str() + "A";
    // println!("slns: {from} {to} {sln1}  {sln2}  {:?}",diff);

    // if they are the same, must be legal, only return one here
    if sln1.eq(&sln2) {
        return vec![sln1];
    }

    // could we have illegal case here?
    if start.x==0 && end.y==3 {
        // remove one of the cases...
        return vec![ sln2 ];
    }
    if end.x==0 && start.y==3 {
        // remove the other case...
        return vec![ sln1 ];
    }
    // otherwise return both
    return vec![sln1,sln2];

    //     >0 => "^".repeat(diff.y.abs() as usize).as_str(),

    // if diff.y < 0 { command = command + "^".repeat(diff.y.abs() as usize).as_str(); }
    // if diff.x < 0 { command = command + "<".repeat(diff.x.abs() as usize).as_str(); }
    // if diff.x > 0 { command = command + ">".repeat(diff.x as usize).as_str(); }
    // if diff.y > 0 { command = command + "v".repeat(diff.y as usize).as_str(); }
    // return command;
}

fn move_on_num_keypad(code: &String) -> String {
    let mut moves = String::new();

    // Assume we're always on A, since we alway pressed an A last
    let mut pos = 'A';
    for c in code.chars() {
        moves.push_str(num_keypad_move(pos, c)[0].as_str());
        // moves.push_str("A");
        // println!(" from {pos} to {c}: {moves}");
        pos = c;
    }

    return moves;
}

////////////////////////////////////
/////////  ARROW KEYPAD //////////////
////////////////////////////////////

fn arrow_keypad_char_to_pos(from: char) -> geo::Coord<i32> {
    let y = match from {
        '^'|'A' => 0,
        _ => 1
    };
    let x = match from {
        '<' => 0,
        '^'|'v' => 1,
        _ => 2
    };
    return coord!{x:x,y:y};
}




fn arrow_keypad_move(from: char, to: char) -> String {

    // if cache.contains_key(&(from,to)) {
    //     return (*cache.get(&(from,to)).unwrap()).clone();
    // }
    let start = arrow_keypad_char_to_pos(from);
    let end = arrow_keypad_char_to_pos(to);
    let mut command = String::new().to_owned();

    let diff = end-start;
    if diff.y > 0 { command = command + "v".repeat(diff.y as usize).as_str(); }
    if diff.x < 0 { command = command + "<".repeat(diff.x.abs() as usize).as_str(); }
    if diff.x > 0 { command = command + ">".repeat(diff.x as usize).as_str(); }
    if diff.y < 0 { command = command + "^".repeat(diff.y.abs() as usize).as_str(); }

    return command;

}


fn arrow_keypad_move_p2(from: char, to: char) -> Vec<String> {

    let start = arrow_keypad_char_to_pos(from);
    let end = arrow_keypad_char_to_pos(to);
    // let mut command = String::new().to_owned();

    let diff = end-start;

    // println!("  arrow_keypad_move_p2 {:?} {:?} {:?} ", from, to, diff);
    // println!("  arrow_keypad_move_p2 {:?} {:?} {:?} ", start, end, diff);
    // if diff.y > 0 { command = command + "v".repeat(diff.y as usize).as_str(); }
    // if diff.x < 0 { command = command + "<".repeat(diff.x.abs() as usize).as_str(); }
    // if diff.x > 0 { command = command + ">".repeat(diff.x as usize).as_str(); }
    // if diff.y < 0 { command = command + "^".repeat(diff.y.abs() as usize).as_str(); }

    let mut hsteps = (if diff.x<0 {"<"} else {">"}).repeat(diff.x.abs() as usize);
    let mut vsteps = (if diff.y<0 {"^"} else {"v"}).repeat(diff.y.abs() as usize);


    // println!("slns: {from} {to} H{hsteps}  V{vsteps}  {:?}",diff);
    let sln1 = vsteps.to_string() + hsteps.as_str() + "A";
    let sln2 = hsteps.to_string() + vsteps.as_str() + "A";
    // println!("slns: {from} {to} {sln1}  {sln2}  {:?}",diff);

    // if they are the same, must be legal, only return one here
    if sln1.eq(&sln2) {
        return vec![sln1];
    }

    // could we have illegal case here?
    if start.y==0 && end.x==0 {
        // remove one of the cases...
        return vec![ sln1 ];
    }
    if start.x==0 && end.y==0 {
        // remove the other case...
        return vec![ sln2 ];
    }
    // otherwise return both
    return vec![sln1,sln2];


    // return command;
}


fn move_on_arrow_keypad(code: &String) -> String {
    let mut moves = String::new();

    // Assume we're always on A, since we alway pressed an A last
    let mut pos = 'A';
    for c in code.chars() {
        moves.push_str(arrow_keypad_move(pos, c).as_str());
        moves.push_str("A");
        // println!(" from {pos} to {c}: {moves}");
        pos = c;
    }

    return moves;
}


fn move_on_arrow_keypad_depth_inner(pos: char, dest: char, depth: i32, moves: &String, cache : &mut HashMap<(char,char,i32),i64>) -> i64 {
    let mut total = 0i64;
    if (depth == 1) { return moves.len() as i64; }


    // Assume we're always on A, since we alway pressed an A last
    let mut curpos = 'A';
    for c in moves.chars() {
        total += move_on_arrow_keypad_depth(curpos, c, depth-1, cache);
        // println!(" from {pos} to {c}: {moves}");
        curpos = c;
    }


    return total;
}
fn move_on_arrow_keypad_depth(pos: char, dest: char, depth: i32, cache : &mut HashMap<(char,char,i32),i64>) -> i64 {

    // println!("move_on_arrow_keypad_depth from {pos} to {dest} depth {depth}");

    if cache.contains_key(&(pos,dest,depth)) {
        // println!("  CACHE HIT {}",*cache.get(&(pos,dest,depth)).unwrap());
        return *cache.get(&(pos,dest,depth)).unwrap();
    }

    let mut moves = arrow_keypad_move_p2(pos, dest);
    println!("moves: {pos} {dest} {depth}:  {:?}", moves);
    let mut ans = move_on_arrow_keypad_depth_inner(pos, dest, depth, &moves[0], cache);

    if moves.len() == 2 {
        let a2 =  move_on_arrow_keypad_depth_inner(pos, dest, depth, &moves[1], cache);
        ans = ans.min(a2);
    }
    
    cache.insert((pos,dest,depth), ans);

    return ans;

}




pub fn process_lines(lines:Vec<String>) -> u64 {
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers

    if (true) {
        println!("{:?}", num_keypad_move('A', '0'));
        println!("{:?}", num_keypad_move('0', '2'));
        println!("{:?}", num_keypad_move('2', '9'));
        println!("{:?}", num_keypad_move('9', 'A'));
        println!("{:?}", num_keypad_move('0', '1'));
        println!("{:?}", num_keypad_move('1', '0'));

        println!("{:?}", arrow_keypad_move_p2('<', '^'));
        println!("{:?}", arrow_keypad_move_p2('^', '<'));

        // return 0u64;

    }






    
    let mut map : Vec<Vec<char>> = Vec::new();
    let mut nummap : Vec<Vec<i32>> = Vec::new();
    let mut blist : VecDeque<geo::Coord<i32>> = VecDeque::new();

    let mut codes : Vec<String> = Vec::new();

    let mut score = 0;

    let mut cache : HashMap<(char,char,i32),i64> = HashMap::new();

    // Do the numeric key pad
    // let mut pos = 'A';
    let mut allmoves : String = String::new();
    for l in lines {
        codes.push(l.clone().to_string());

        // P1 work =======================

        println!("Code: {l}");
        // let mut pos = 'A';
            let moves = move_on_num_keypad(&l);
            // println!(" from {pos} to {c}: {moves}");
            // pos = c;
        // }

        println!(" Moves for  {l}:    {moves}");
        allmoves.push_str(moves.as_str());

        // Move on arrow key pad twice
        let remote_moves_1 = move_on_arrow_keypad(&moves);
        let remote_moves_2 = move_on_arrow_keypad(&remote_moves_1);
        println!(" remote for {l} 1:  {remote_moves_1}");
        println!(" remote for {l} 2:  {remote_moves_2}");

        // score this
        let num_val: i32 = l.clone().replace("A","").parse().expect("parse error");
        let len = remote_moves_2.len() as i32;
        let this_score = num_val*len;
        println!(" score for  {l} is {len}*{num_val}={this_score}");
        // score += this_score as u64;


        let depth_p1 = 2;
        let depth_p2 = 25;
        let depth = depth_p2;

        let mut newlen = 0;
        let mut pos = 'A';
        for c in l.chars() {

            let moves = num_keypad_move(pos, c);
            println!("STARTING WITH MOVES from {pos} to {c} result: {:?}", moves);
            let lengths: Vec<i64> = moves.clone().iter().map(|m| {
                let mut size = 0i64;
                let mut pos2 = 'A';
                for c2 in m.chars() {
                // let mut longcode = m2;
                    let l = move_on_arrow_keypad_depth(pos2,c2,depth,&mut cache);
                    // println!("MOVING {pos2} {c2} top={depth} size={l}");
                    size += l;
                    pos2 = c2;
                }
                // for n in 0..2 {
                //     longcode = move_on_arrow_keypad(&longcode);
                //     println!("     iter {n} len={}",longcode.len());
                // }
                // let l = longcode.len();
                size
            }).collect();
            let mut min: i64 = std::i64::MAX;
            println!("moves are {:?} lengths {:?}",moves,lengths);
            for l in lengths {
                min = min.min(l as i64);
            }
            println!("min len is {min}");
            newlen += min;

            pos = c;

        }
        let new_score = num_val as i64*newlen;
        println!(" new score for  {l} is {newlen}*{num_val}={new_score}");
        score += new_score as u64;


    }
    // println!("All Moves: {allmoves}");

    if (false) {
        println!("moves for <<^^:  {}", move_on_arrow_keypad(&"<<^^".to_string()));
        println!("moves for ^^<<:  {}", move_on_arrow_keypad(&"^^<<".to_string()));
        println!("moves for <<^^A: {}", move_on_arrow_keypad(&"<<^^A".to_string()));
        println!("moves for ^^<<A: {}", move_on_arrow_keypad(&"^^<<A".to_string()));
        println!("2xves for <<^^A: {}", move_on_arrow_keypad(&move_on_arrow_keypad(&"<<^^A".to_string())));
        println!("2xves for ^^<<A: {}", move_on_arrow_keypad(&move_on_arrow_keypad(&"^^<<A".to_string())));
    }

    println!("test result:                 {score}");


    // unsafe { score = LOW_SCORE; }


    // let good_cells = count_cells_on_best_paths(&mut map, start_pos, end_pos);
    // println!("good cells count is {good_cells}");


    return score as u64;
}