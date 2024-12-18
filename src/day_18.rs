// use regex::{bytes, Regex};
// use std::collections::HashSet;
// use multimap::MultiMap;

use std::{collections::{vec_deque, HashMap, HashSet, VecDeque}, i32::MAX};

// use std::collections::HashSet;
// use std::collections::HashMap;
use geo::{coord};
// use multimap::MultiMap;



const WALL:char = '#';
const SPACE:char = '.';
const START:char = 'S';
const END:char = 'E';

const EAST: geo::Coord<i32> = geo::coord!{x:1,y:0};
const NORTH: geo::Coord<i32> = geo::coord!{x:0,y:-1};
const SOUTH: geo::Coord<i32> = geo::coord!{x:0,y:1};
const WEST: geo::Coord<i32> = geo::coord!{x:-1,y:0};
const NONE: geo::Coord<i32> = geo::coord!{x:0,y:0};

#[derive(Debug,PartialEq,Clone,Copy)]
enum MoveType {
    F = 1,
    L = 2,
    R = 3,
}
impl MoveType {
    pub fn is_turn(&self) -> bool {
        return self==&MoveType::L || self==&MoveType::R;
    }
}

#[derive(Debug,PartialEq,Clone)]
struct Route {
    pos : geo::Coord<i32>,
    orient : geo::Coord<i32>,
    path : Vec<MoveType>,
    history : HashSet<geo::Coord<i32>>,
    score : i32
}

impl Route {
    // Will make one step, but for turns it includes a turn and step forward
    pub fn proceed( &mut self, mt : MoveType ) {
        // println!("proceeding {:?}", mt);
        self.history.insert(self.pos);
        self.orient = step_orientation(self.orient, &mt);
        self.pos = step_pos(self.pos, self.orient, &mt);
        self.path.push(mt);
        if mt.is_turn() {
            self.score += 1000;
            // recurse with a step forward as well
            self.proceed(MoveType::F);
        } else {
            self.score += 1;
        }
    }

    pub fn proceed_single( &mut self, mt : MoveType ) {
        // println!("proceeding {:?}", mt);
        self.history.insert(self.pos);
        self.orient = step_orientation(self.orient, &mt);
        self.pos = step_pos(self.pos, self.orient, &mt);
        self.path.push(mt);
        
    }

    pub fn recalc_score( &self ) -> i32 {
        let mut s = 0;
        for mt in self.path.clone() {
            if mt.is_turn() { s+= 1000; }
            else { s += 1; }
        }
        return s;
    }
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

fn find_pos(map: &mut Vec<Vec<char>>, to_find: char ) -> geo::Coord<i32>  {
    let mut found_pos: geo::Coord<i32> = geo::coord!{x:0,y:0};
    for y in 0..map.len() {
        if map[y].contains(&to_find) {
            let x = map[y].iter().position(|&r| r == to_find).unwrap();
            return  geo::coord!{x:x as i32,y:y as i32};
        }
    }
    assert!(false);
    return geo::coord!{x:-1,y:-1};
}

fn test_fwd(map: &mut Vec<Vec<char>>, r: &Route, test_orientation: geo::Coord<i32>) -> bool {
    let next = r.pos + test_orientation;

    // if we've been in that step, consider it a no
    if r.history.contains(&next) { return false; }

    let c = get_loc(map, &next);
    // todo: add check for loops

    let ok = c == SPACE || c==END;
    // println!("test_fwd {:?} {:?} {:?} {c} {ok}", r.pos, test_orientation, next );
    return ok;
}

// fn check_end_move(map: &mut Vec<Vec<char>>, r: &Route, br : Vec<MoveType>) -> Option<MoveType> {

//     for b in br {
//         let test_dir = turn(r.orient, b);
//         let test_pos = r.pos + test_dir;
//         let c = get_loc(map, &next);
//         if c == END { return Some(b); }
//     }
//     return None();
// }



fn turn(cur: geo::Coord<i32>, mt:&MoveType) -> geo::Coord<i32> {
    return match (cur,mt) {
        (EAST, &MoveType::L) => NORTH,
        (NORTH,&MoveType::L) => WEST,
        (WEST, &MoveType::L) => SOUTH,
        (SOUTH,&MoveType::L) => EAST,
        (EAST, &MoveType::R) => SOUTH,
        (NORTH,&MoveType::R) => EAST,
        (WEST, &MoveType::R) => NORTH,
        (SOUTH,&MoveType::R) => WEST,
        _ => cur
    }
}

fn find_branches(map: &mut Vec<Vec<char>>, r: &Route) -> Vec<MoveType> {
    let mut result : Vec<MoveType> = Vec::new();

    // Simple, test forward
    if test_fwd(map, r, r.orient) { result.push(MoveType::F);};
    // check left
    let left_orientation = turn(r.orient,&MoveType::L);
    if test_fwd(map, r, left_orientation) { result.push(MoveType::L);};
    let right_orientation = turn(r.orient,&MoveType::R);
    if test_fwd(map, r, right_orientation) { result.push(MoveType::R);};

    return result;
} 

fn step_orientation(cur: geo::Coord<i32>, mt:&MoveType) -> geo::Coord<i32> {
    if *mt==MoveType::L || *mt==MoveType::R { return turn(cur, &mt)}
    return cur;
}
fn step_pos(cur: geo::Coord<i32>, orientation: geo::Coord<i32>, mt:&MoveType) -> geo::Coord<i32> {
    if *mt==MoveType::F { return cur+orientation; }
    return cur;
}


// This will run a path with all the first moves, until it reaches end or dead end
// At branches, it will clone and afterwards finish all the clones
fn continue_routes(map: &mut Vec<Vec<char>>, 
    low_cost_cache: &mut HashMap<(geo::Coord<i32>,geo::Coord<i32>), i32>, 
    route: &mut Route, end_loc: geo::Coord<i32>) -> Vec<Route> {
    
    let mut all_paths: Vec<Route> = Vec::new();
    let mut future_test_paths: Vec<Route> = Vec::new();

    let mut cur_route = route.clone();

    
    let mut is_dead_end = false;

    loop {
        if cur_route.pos == end_loc {
            // found the end, take that move and end this processing
            let score = cur_route.recalc_score();
            // println!("**** FOUND END ***** score={score} path={:?}", cur_route.path);
            println!("**** FOUND END ***** score={score} path count={:?}", cur_route.path.len());
            check_score(score,&cur_route.path);

            if future_test_paths.is_empty() {
                println!("No more paths to run, breaking");
                break;
            } else {
                let mut t = future_test_paths.pop().unwrap();
                cur_route = t;
            }
        }


        // check low cost cache



        let mut br = find_branches(map, &cur_route);
        // println!("pos {:?} or {:?} branches {:?}", cur_route.pos, cur_route.orient, br);


        // check for dead end
        // if so, return without our path
        if br.len() == 0 {
            is_dead_end = true;
            // println!("    DEAD END");

            if future_test_paths.is_empty() {
                println!("No more paths to run, breaking");
                break;
            } else {
                let mut t = future_test_paths.pop().unwrap();
                cur_route = t;
            }
        }

        while br.len() > 1 {
            // queue up additional paths
            let b = br.pop().unwrap();
            // println!("queuing new route for {:?}", b);
            let mut new_path = cur_route.clone();
            new_path.proceed(b);
            // save it for later processing
            future_test_paths.push(new_path);
            // println!("queuing new route for {:?} len={}", b, future_test_paths.len());
        }

        if br.len() == 1 {
            // make 1 move and continue
            let b = br.pop().unwrap();
            cur_route.proceed(b);

            // see if our pos/or is in the low cost cache
            let cache_val = *low_cost_cache.get( &(cur_route.pos, cur_route.orient) ).unwrap_or(&std::i32::MAX);
            if cache_val < cur_route.score {
                // someone was here cheaper, abandon
                if future_test_paths.is_empty() {
                    println!("No more paths to run, breaking");
                    break;
                } else {
                    let mut t = future_test_paths.pop().unwrap();
                    cur_route = t;
                }
            }
            if cur_route.score < cache_val {
                low_cost_cache.insert( (cur_route.pos, cur_route.orient), cur_route.score );
            }



        }
        // else {
        //     // multiple branches
        //     let b = br.pop().unwrap();
        //     println!("... following {:?}", b);
        //     route.proceed(b);
        // }

    }

    return all_paths;
}


fn start_routes(map: &mut Vec<Vec<char>>, start_pos: geo::Coord<i32>, end_pos:  geo::Coord<i32> ) -> Vec<&Route> {
    let mut all_paths: Vec<Vec<MoveType>> = Vec::new();
    // always start facing east
    let mut initial = Route{
        pos: start_pos,
        orient: EAST,
        path: Vec::new(),
        history: HashSet::new(),
        score: 0
        };
    

    let mut low_cost_cache: HashMap<(geo::Coord<i32>,geo::Coord<i32>), i32> = HashMap::new();

    let br = find_branches(map, &initial);
    println!("initial branches {:?}", br);

    let all_routes: Vec<&Route> = Vec::new();

    // start all the initial branches
    for b in br {
        // create each route and make first step away from start
        let mut route = initial.clone();
        route.proceed(b);

        // we push all routes now, but may remove them later if dead end
        // all_routes.push(&route);



        // return all paths from here, or None if this is a dead end
        let good_routes: Vec<Route> = continue_routes(map, &mut low_cost_cache, &mut route, end_pos);
        // all_routes.extend(good_routes);
    }

    return all_routes;
}

static mut LOW_SCORE : i32 = std::i32::MAX;
static mut LOW_SCORE_MOVES : Vec<Vec<MoveType>> = Vec::new();

pub fn check_score(score: i32, moves : &Vec<MoveType>) {
    unsafe {
        let test = LOW_SCORE.min(score);
        if test != LOW_SCORE {
            println!("Found new low score: {test}");
            LOW_SCORE = test;
            LOW_SCORE_MOVES.clear();
        }
        if score == LOW_SCORE {

            LOW_SCORE_MOVES.push(moves.clone());
            println!("Adding another path to low score, count={}", LOW_SCORE_MOVES.len());
        }
        
    }
}

pub fn count_cells_on_best_paths(map: &mut Vec<Vec<char>>, start_pos: geo::Coord<i32>, end_pos:  geo::Coord<i32>) -> i32 {
    let mut cells_hit : HashSet<geo::Coord<i32>> = HashSet::new();


    cells_hit.insert(start_pos);
    cells_hit.insert(end_pos);

    unsafe {

        for path in LOW_SCORE_MOVES.clone() {

            let mut rt = Route{
                pos: start_pos,
                orient: EAST,
                path: Vec::new(),
                history: HashSet::new(),
                score: 0
                };

            println!("starting of another rt");
            let last_was_turn = false;
            for mt in path.clone() {
                rt.proceed_single(mt);
                println!("move {:?} cell hit is {:?}", mt, rt.pos);

                cells_hit.insert(rt.pos);

            }


        }



    }


    return cells_hit.len() as i32;

} 



fn get_map(map: &mut Vec<Vec<i32>>, loc: &geo::Coord<i32>) -> i32  {
    let r = map.get(loc.y as usize);
    if r == None { return -1; };
    return *r.unwrap().get(loc.x as usize).unwrap_or(&-1);
}


fn flood_fill(map: &mut Vec<Vec<i32>>) {
    let dirs = vec![ 
        coord!{x:-1,y:0},
        coord!{x:1,y:0},
        coord!{x:0,y:-1},
        coord!{x:0,y:1},
    ];
    // let mut cur = 1;
    map[0][0] = 0;
    let mut workqueue : VecDeque<geo::Coord<i32>> = VecDeque::new();
    workqueue.push_back(coord!{x:0,y:0});

    while ! workqueue.is_empty() {
        let pos = workqueue.pop_front().unwrap();
        let v = get_map(map,&pos);
        for d in dirs.iter() {
            let pos2 = pos + *d;
            if get_map(map, &pos2) == std::i32::MAX {
                // set to the new value
                map[pos2.y as usize][pos2.x as usize] = v+1;
                workqueue.push_back(pos2);
            }
        }
    }

}


fn reset_map(map : &mut Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != -1 {
                map[y][x] = std::i32::MAX;
            }
        }
    }
}



const SAMPLE_SIZE: geo::Coord<i32> = coord!{x:7,y:7};
const SAMPLE_BYTE_COUNT: i32 = 12;

const REAL_SIZE: geo::Coord<i32> = coord!{x:71,y:71};
const REAL_BYTE_COUNT: i32 = 1024;

pub fn process_lines(lines:Vec<String>) -> u64 {
  
    // iterate the chars and determine the size of the entire FS
    // Size of FS is sum of numbers

    let grid_size = SAMPLE_SIZE;
    let init_byte_count = SAMPLE_BYTE_COUNT;
    let grid_size = REAL_SIZE;
    let byte_count = REAL_BYTE_COUNT;


    let mut map : Vec<Vec<char>> = Vec::new();
    let mut intmap : Vec<Vec<i32>> = Vec::new();
    let mut blist : VecDeque<geo::Coord<i32>> = VecDeque::new();

    for n in 0..grid_size.y {
        map.push(vec![SPACE; grid_size.x as usize]);
        intmap.push(vec![std::i32::MAX; grid_size.x as usize]);
    }
    
    print_map(&map);

    let mut count = 0;
    for l in lines {
        let row: Vec<&str> = l.trim().split(",").collect();
        let c =  coord!{ x: row[0].parse().unwrap(), 
            y: row[1].parse().unwrap() };
        println!("{l}   {:?}  {count}",c);
        if blist.len() < init_byte_count as usize {
            map[c.y as usize][c.x as usize] = WALL;
            intmap[c.y as usize][c.x as usize] = -1;
        }
        blist.push_back(c);
        count += 1;
    }


    flood_fill(&mut intmap);
    let end_count = get_map(&mut intmap, 
        &coord!{x:grid_size.x-1,y:grid_size.y-1});
    println!("after end_count = {end_count}");


    // remove the first chunk of bytes so testlist has the remainder to test
    let mut testlist = blist.clone();
    for n in 0..init_byte_count {
        testlist.pop_front();
    }

    for n in testlist.clone() { 
        // println!("coord {:?}", n);
    }

    loop {
        reset_map(&mut intmap);

        let co = testlist.pop_front();
        if co.is_none() {
            println!("No more coords left, error!");
            return 0u64;
        }
        let c= co.unwrap();
        intmap[c.y as usize][c.x as usize] = -1;

        flood_fill(&mut intmap);
        let end_count = get_map(&mut intmap, 
            &coord!{x:grid_size.x-1,y:grid_size.y-1});
        println!("after coord {:?} end_count = {end_count}", c);

        if end_count == std::i32::MAX {
            println!("map cound not be flooded!  coord={:?}", c);
            return 0u64;
        }

    }


    let end_count = get_map(&mut intmap, 
        &coord!{x:grid_size.x-1,y:grid_size.y-1});

    println!("after end_count = {end_count}");

    print_map(&map);

    for n in blist { 
        // println!("coord {:?}", n);
    }

    println!("after end_count = {end_count}");
    println!("after end_count = {}", end_count-1);


    // fill_dead_ends(&map);

    // find the start location
    // let mut start_pos = find_pos(&mut map, START);
    // let mut end_pos = find_pos(&mut map, END);
    // println!("start {:?} end {:?}", start_pos, end_pos);

    // start_routes(&mut map, start_pos, end_pos);



    let mut score = 0;
    // unsafe { score = LOW_SCORE; }


    // let good_cells = count_cells_on_best_paths(&mut map, start_pos, end_pos);
    // println!("good cells count is {good_cells}");


    return score as u64;
}