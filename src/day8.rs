// use regex::{bytes, Regex};
use std::collections::HashSet;
use multimap::MultiMap;


fn on_map(x:i32, y:i32, dim:(usize,usize)) -> bool {
    return x>=0 && y>=0 && x<dim.0 as i32 && y<dim.1 as i32;
}


fn diff_locs(loc1 : (i32,i32), loc2 : (i32,i32)) -> (i32,i32) {
    return (loc2.0-loc1.0,loc2.1-loc1.1);
}

// static mut UNIQUE_ANTENNA : HashSet<(i32,i32)> = HashSet::new();

pub fn find_antinodes( loc1 : (i32,i32), loc2 : (i32,i32), dim:(usize,usize), unique_antennas : &mut HashSet<(i32,i32)> ) {

    let diff = diff_locs(loc1, loc2);

    // iterate less than
    let mut an = (loc1.0-diff.0,loc1.1-diff.1);
    loop {
        let an_on_map = on_map(an.0,an.1,dim);
        if an_on_map { 
            unique_antennas.insert(an);
            an = (an.0-diff.0,an.1-diff.1);
        } 
        else { break };
    }
    an = (loc2.0+diff.0,loc2.1+diff.1);
    loop {
        let an_on_map = on_map(an.0,an.1,dim);
        if an_on_map { 
            unique_antennas.insert(an);
            an = (an.0+diff.0,an.1+diff.1);
        } 
        else { break };
    }

    // let an2 = (loc2.0+diff.0,loc2.1+diff.1);

    // let an1_on_map = on_map(an1.0,an1.1,dim);
    // let an2_on_map = on_map(an2.0,an2.1,dim);
    // println!("an1 {:?} on map {an1_on_map}", an1);
    // println!("an2 {:?} on map {an2_on_map}", an2);

    // unsafe {
    // if an1_on_map { unique_antennas.insert(an1);}
    // if an2_on_map { unique_antennas.insert(an2);}
    // }


}


pub fn process_lines(lines:Vec<String>) -> u64 {

    let mut count = 0;
    // unsafe {
    //     UNIQUE_ANTENNA = HashSet::new();
    // }
    let mut unique_antennas: HashSet<(i32,i32)> = HashSet::new();
    // let map : VecMul<str> = Vec::new();
    let mut antenna_map : MultiMap<char,(i32,i32)> = MultiMap::new();

    let map_dim = (lines.len(),lines[0].len());

    let mut y = 0;
    for l in lines {
        println!("row: {y} {l}");

        // unique_antennas.push(l.as_str());

        // Parse line in one char at a time
        let mut x = 0;
        for c in l.as_str().chars() {
            if c == '.' { 
                x += 1;
                continue 
            }

            // otherwise put the coord in the multimap
            antenna_map.insert(c,(x,y));
            x += 1;
        }
        y += 1;
    }

    println!("antenna map {:?}", antenna_map);

    for ants in antenna_map.keys() { 
        println!("antenna {ants}");
        if antenna_map.get_vec(ants).unwrap().len() == 1 { 
            println!("ONLY ONE ANTENNA FOR {ants}");
            continue;
        }
        for locs in antenna_map.get_vec(ants) {
            println!("antenna {:?}", locs);
            let len = locs.len();
            for x in 0..len {
                // add the actual antenna location
                unique_antennas.insert(locs[x]);
                for y in x+1..len {
                    println!( "compare {x} {y} {len} {:?} {:?}", locs[x], locs[y]);
                    find_antinodes(locs[x], locs[y], map_dim, &mut unique_antennas);
                }

            }

        }
    }


    unsafe {
        count = unique_antennas.len();
    }
    return count as u64;

}