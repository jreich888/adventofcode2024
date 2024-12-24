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

fn find_all_connected(conn_map: &MultiMap<String,String>,k: &String) -> Vec<String> {
    let mut connset : Vec<String> = Vec::new();
    connset.push( (*k.clone()).to_string());

    // what are all of k's connections
    let conns = conn_map.get_vec(k).unwrap();
    for c in conns {
        // test to see if C is are connected to everyone so far
        let c_conns = conn_map.get_vec(c).unwrap();
        let mut all_found = true;
        for existing_friend in connset.iter() {
            if !c_conns.contains(existing_friend) { 
                all_found = false; 
                break; 
            }
        }
        if all_found {
            // add them to the connset
            connset.push( (*c.clone()).to_string() );
        }
    }

    return connset;

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

    // build set of player connections of exactly 3
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


    let mut largest_network : Vec<String> = Vec::new();
    for k in conn_map.keys() {

        let all_connected = find_all_connected(&conn_map,k);

        // good enough?  

        if all_connected.len() > largest_network.len() {
            largest_network = all_connected.clone();
            largest_network.sort();
        }
    }

    
    println!("largest network is len={}  {:?}", largest_network.len(), largest_network);

    let mut password = format!("{:?}",largest_network).replace("\"", "").replace(" ","");
    println!("password is {password}");



   
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers


    // unsafe { score = LOW_SCORE; }



    // let good_cells = count_cells_on_best_paths(&mut map, start_pos, end_pos);
    // println!("good cells count is {good_cells}");


    return score as u64;
}