// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use core::num;
use std::{collections::{vec_deque, BTreeMap, HashMap, HashSet, VecDeque }, hash::Hash, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
use lazy_static::lazy_static;
use multimap::MultiMap;

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


fn move_on_arrow_keypad_depth(pos: char, dest: char, depth: i32, cache : &mut HashMap<(char,char,i32),i64>) -> i64 {

    println!("move_on_arrow_keypad_depth from {pos} to {dest} depth {depth}");

    if cache.contains_key(&(pos,dest,depth)) {
        println!("  CACHE HIT {}",*cache.get(&(pos,dest,depth)).unwrap());
        return *cache.get(&(pos,dest,depth)).unwrap();
    }

    let mut moves = arrow_keypad_move(pos, dest) + "A";
    println!("moves: {pos} {dest} {depth}:  {moves}");

    if (depth == 1) { return moves.len() as i64; }

    let mut total = 0i64;

    // Assume we're always on A, since we alway pressed an A last
    let mut curpos = 'A';
    for c in moves.chars() {
        total += move_on_arrow_keypad_depth(curpos, c, depth-1, cache);
        // println!(" from {pos} to {c}: {moves}");
        curpos = c;
    }

    cache.insert((pos,dest,depth), total);

    return total;
}

fn mix(n1:i64, n2:i64) -> i64 { n1^n2 }
fn prune(n1:i64) -> i64 { n1 % 16777216 }


fn calc_secret_number(n: i64 ) -> i64 {
    let n1 = n * 64i64;
    let mut sn = prune(mix(n, n1));
    let n2 = sn / 32i64;
    sn = prune(mix(sn, n2));
    let n3 = sn * 2048i64;
    sn = prune(mix(sn, n3));

    return sn;


}



pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut score = 0u64;

    let mut net_count = 0;
    let mut conn_map : MultiMap<String,String> = MultiMap::new();
    let mut pc_map : HashMap<String,i32> = HashMap::new();
    let mut net_map: HashMap<i32, HashSet<String>> = HashMap::new();
    for l in lines {
        let pair : Vec<&str> = l.split("-").collect();
        // check if either in pc map
        let one_net =  pc_map.get(pair[0]);
        let two_net =  pc_map.get(pair[1]);

        println!("parsed {} and {} nets {:?} {:?}", pair[0], pair[1], one_net, two_net);

        conn_map.insert(pair[0].to_string(), pair[1].to_string());
        conn_map.insert(pair[1].to_string(), pair[0].to_string());

        let mut new_net = -1;
        if one_net.is_some() { new_net = *one_net.unwrap(); }
        else if two_net.is_some() { new_net = *two_net.unwrap(); }

        if new_net < 0 {
            new_net = net_count;
            net_count += 1;
            println!("found new net {new_net}");
            net_map.insert(new_net, HashSet::new());
        } else {
            println!("found existing net {new_net}");
        }

        pc_map.insert(pair[0].to_string(), new_net);
        pc_map.insert(pair[1].to_string(), new_net);

        let net_list = net_map.get_mut(&new_net).unwrap();

        net_list.insert( pair[0].to_string() );
        net_list.insert( pair[1].to_string() );

    }

    for k in net_map.iter() {
        println!( "network {} has {:?}", k.0, k.1);
    }

    let mut playersets : HashSet<(&String,&String,&String)> = HashSet::new();

    for k in conn_map.keys() {
        // for k see how many connections they have
        let conns = conn_map.get_vec(k).unwrap();
        for c in conns {
            // get these connections
            let c_conns = conn_map.get_vec(c).unwrap();
            for c2 in c_conns {
                // see if k and C2 are connecte
                if conns.contains(c2) {
                    // found a threesome 
                    let mut new_players = vec![k,c,c2];
                    new_players.sort();
                    let new_key = (new_players[0],new_players[1],new_players[2]);
                    playersets.insert(new_key);

                }
            }
        }
    }

    println!("total lan games {}", playersets.len());
    for pl in playersets.clone() {
        println!("Players {:?}", pl);
        // if pl.0.starts_with("t") || pl.1.starts_with("t") || pl.2.starts_with("t") {
        //     println!("Found possible set {:?}", pl);
        //     score += 1;
        // }

    }

    for pl in playersets {
        // println!("Players {:?}", pl);
        if pl.0.starts_with("t") || pl.1.starts_with("t") || pl.2.starts_with("t") {
            println!("Found possible set {:?}", pl);
            score += 1;
        }

    }





   
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers


    // unsafe { score = LOW_SCORE; }



    // let good_cells = count_cells_on_best_paths(&mut map, start_pos, end_pos);
    // println!("good cells count is {good_cells}");


    return score as u64;
}