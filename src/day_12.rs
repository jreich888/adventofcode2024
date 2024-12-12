// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::collections::HashSet;


fn get_cell_at(plot_map:&Vec<Vec<char>> ,dims:(i32, i32),x:i32,y:i32) -> char {
    // either get elev at coord, or -1 if off map
    let row =plot_map.get(y as usize);
    match row {
        None => return '.',
        _ => return *row.unwrap().get(x as usize).unwrap_or(&'.')
    }
}


fn expand_region(plot_map:&Vec<Vec<char>> ,dims:(i32, i32),x:i32,y:i32, crop: char, region: &mut HashSet<(i32,i32)>) {

    // println!("expand region: {x} {y} {crop}");

    for dx in -1..2 {
        // could skip check but I like it
        if dx==0 {continue;}

        let nbr1 = get_cell_at(plot_map, dims, x+dx, y);
        // println!("  expand region nbr1  {nbr1}");
        if nbr1 == crop { 
            if !region.contains(&(x+dx,y)) {
                region.insert((x+dx,y));
                expand_region(plot_map, dims, x+dx, y, crop, region); 
            }
        }
        let nbr2 = get_cell_at(plot_map, dims, x, y+dx);
        // println!("  expand region nbr2  {nbr2}");
        if nbr2 == crop { 
            if !region.contains(&(x,y+dx)) { 
                region.insert((x,y+dx));
                expand_region(plot_map, dims, x, y+dx, crop, region); 
            }
        }
    }
}



fn find_region(plot_map:&Vec<Vec<char>> ,dims:(i32, i32),x:i32,y:i32) -> HashSet<(i32,i32)> {
    let letter = get_cell_at(plot_map, dims,x,y);

    let mut region : HashSet<(i32,i32)> = HashSet::new();

    region.insert((x,y));

    expand_region(plot_map, dims, x, y, letter, &mut region);

    return region;

}


fn calc_perim( region: &HashSet<(i32,i32)> ) -> i32 {
    // perimeter is sum of sides of cells without neighbors in each
    let mut perim = 0;
    for c in region.clone() {
        if is_edge(region,c.0,c.1,LEFT) { perim += 1; }
        if is_edge(region,c.0,c.1,RIGHT) { perim += 1; }
        if is_edge(region,c.0,c.1,TOP) { perim += 1; }
        if is_edge(region,c.0,c.1,BOTTOM) { perim += 1; }
        // if !region.contains(&(c.0-1,c.1)) { perim += 1; }
        // if !region.contains(&(c.0+1,c.1)) { perim += 1; }
        // if !region.contains(&(c.0,c.1-1)) { perim += 1; }
        // if !region.contains(&(c.0,c.1+1)) { perim += 1; }
    }
    return perim;
}


#[derive(Eq, Hash, PartialEq)]
enum SideType {
    LEFT=1, RIGHT=2, TOP=3, BOTTOM=4
}
fn test_side_type() {
    let mut sides : HashSet<SideType> = HashSet::new();
    sides.insert(SideType::LEFT);
}

// Future, use enum type like above
const LEFT : i32 = 0;
const RIGHT : i32 = 1;
const TOP : i32 = 2;
const BOTTOM : i32 = 3;

fn is_edge(region: &HashSet<(i32,i32)>, x:i32, y:i32, side:i32 ) -> bool {
    let mut dx = 0;
    let mut dy = 0;
    match side {
        LEFT=> dx=-1,
        RIGHT=> dx=1,
        TOP=> dy=-1,
        BOTTOM=> dy=1,
        _ => dy=1,
    }
    return !region.contains(&(x+dx,y+dy));
}


// Key to P2...
// for any side, the side ID will be the neigboring inline side with the minimum
// X or Y.  So three top sides in a row will have the same side_id (hash key) all
// with the X of the leftmost coordinate
fn calc_side_id(region: &HashSet<(i32,i32)>, x:i32, y:i32, stype:i32) -> (i32,i32,i32) {
    // println!("calc_side_id {x} {y} {stype}");
    let mut dx = 0;
    let mut dy = 0;
    // for this side type, are we iterating up or left?
    if [LEFT,RIGHT].contains(&stype) { dy = -1; } else { dx=-1; }

    let mut minx = x;
    let mut miny = y;
    loop {
        // see if cell above has adjacent side
        // println!("   calc_side_id {minx} {miny} {dx} {dy} {stype}");
        let adjacent_cell = region.contains(&(minx+dx,miny+dy));
        let adjacent_edge = is_edge(region,minx+dx,miny+dy,stype);
        // println!("       calc_side_id adjacent {adjacent}  adjacent_edge {adjacent_edge}");
        if adjacent_cell && adjacent_edge {
            // continue looking in direction
            minx += dx;
            miny += dy;
        } else { break; }
    }

    return (minx,miny,stype);

}


fn calc_sides( region: &HashSet<(i32,i32)> ) -> i32 {
    // perimeter is sum of sides of cells without neighbors in each
    // each side is notated by it's min-x, min-y, and T/L/R/B

    let mut sides : HashSet<(i32,i32,i32)> = HashSet::new();
    for c in region.clone() {
        for s in [LEFT,RIGHT,TOP,BOTTOM] {
        if is_edge(region,c.0,c.1,s) { 
            let side_id = calc_side_id(region,c.0, c.1, s);
            sides.insert(side_id);
         }
        }
    }
    // println!("sides are: {:?}",sides);
    // for s in sides.clone() { println!("   side {:?}", s);}
    return sides.len() as i32;
}



pub fn process_lines(lines:Vec<String>) -> u64 {
  
    let mut plot_map: Vec<Vec<char>> = Vec::new();

    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers
    for l in lines {
        let row = l.chars().collect();
        plot_map.push(row);
    }

    for r in plot_map.clone() {
        let rs: String = r.iter().collect();
        println!("{}", rs);
    }

    let dims: (i32, i32) = (plot_map[0].len() as i32, plot_map.len() as i32);

    if false 
    {
        let region00 = find_region(&plot_map,dims,0,0);
        let region_area = region00.len() as i32;
        let region_perim = calc_perim(&region00);
        let region_sides = calc_sides(&region00);

        println!("region00 area {} perim {} sides {}  ALL {:?}", region_area, region_perim, region_sides, region00);
        return 0u64;
    }

    let mut total_cost =0;

    // union of found regions so far
    let mut found_plots : HashSet<(i32,i32)> = HashSet::new();

    // iterate looking for regions
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            if found_plots.contains(&(x,y)) {continue;}

            let region = find_region(&plot_map,dims,x,y);
            found_plots.extend(region.clone());

            let plant = get_cell_at(&plot_map, dims, x, y);
            let region_area = region.len() as i32;
            let region_perim = calc_perim(&region);
            let region_sides = calc_sides(&region);

            // P1
            // let cost = region_area*region_perim;
            // p2
            let cost = region_area*region_sides;
            total_cost += cost;

            println!("found region {plant} area {region_area} perim {region_perim} sides {region_sides} cost {cost}");
        }
    }
    
   return total_cost as u64;
}