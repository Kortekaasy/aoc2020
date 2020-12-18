use std::collections::HashMap;
// ========================= Challenge Logic ============================
pub fn update_state(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, bool>>>, x: i64, y: i64, z: i64, state: bool) {
    *map.entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(false) = state;
}

pub fn increment_count(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, usize>>>, x: i64, y: i64, z: i64) {
    *map.entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(0) += 1;
}

pub fn get_count(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, usize>>>, x: i64, y: i64, z: i64) -> usize {
    *(map.entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(0))
}

pub fn active(map: &HashMap<i64, HashMap<i64, HashMap<i64, bool>>>, x: i64, y: i64, z: i64) -> bool {
    if let Some(zmap) = map.get(&z) {
        if let Some(ymap) = zmap.get(&y) {
            if let Some(state) = ymap.get(&x) {
                *state
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn print_map(map: &HashMap<i64, HashMap<i64, HashMap<i64, bool>>>, range: i64) {
    // println!("Called with range: {} and map {:?}", range, map);
    for z in (-range / 2)..(range / 2 + 1) {
        println!("z={}", z);
        for y in (-range / 2)..(range / 2 + 1) {
            for x in (-range / 2)..(range / 2 + 1) {
                if active(&map, x, y, z) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

pub fn part1(input: String) -> String {
    let mut map: HashMap<i64, HashMap<i64, HashMap<i64, bool>>> = HashMap::new();

    let mut range = 0;
    // load in input
    input.lines().enumerate().for_each(|(j, l)| {
        range = l.len() as i64;
        let middle = (l.len() / 2) as i64;
        l.chars().enumerate().for_each(|(i, c)| {
            update_state(&mut map, (i as i64) - middle, (j as i64) - middle, 0, c == '#');
        });
    });

    // println!("Before any cycles:");
    // print_map(&map, range);
    // println!("");

    let cycles = 6;
    
    for cyc in 0..cycles {
        let mut new_map: HashMap<i64, HashMap<i64, HashMap<i64, usize>>> = HashMap::new();
        // compute a neighbour count for every square
        for z in map.keys() {
            for k in &[-1, 0, 1] {
                for y in map[z].keys() {
                    for j in &[-1, 0, 1] {
                        for x in map[z][y].keys() {
                            if !active(&map, *x, *y, *z) {
                                continue;
                            }
                            for i in &[-1, 0, 1] {
                                let (c_x, c_y, c_z) = (x + i, y + j, z + k);
                                if c_x == *x && c_y == *y && c_z == *z {
                                    continue;
                                }
                                increment_count(&mut new_map, c_x, c_y, c_z);
                            }
                        }
                    }
                }
            }
        }

        // Check for every count whether that spot will become active
        // for z in new_map.keys() {
        //     for y in new_map[z].keys() {
        //         for x in new_map[z][y].keys() {
        //             if active(&map, *x, *y, *z) {
        //                 // println!("Setting ({}, {}, {}) to {} ({} neighbours active)", x, y, z, new_map[z][y][x] == 2 || new_map[z][y][x] == 3,new_map[z][y][x]);
        //                 update_state(&mut map, *x, *y, *z, new_map[z][y][x] == 2 || new_map[z][y][x] == 3);
        //             } else {
        //                 // println!("Setting ({}, {}, {}) to {} ({} neighbours active)", x, y, z, new_map[z][y][x] == 3, new_map[z][y][x]);
        //                 update_state(&mut map, *x, *y, *z, new_map[z][y][x] == 3);
        //             }
        //         }
        //     }   
        // }
        range += 2;

        for z in (-range / 2)..(range / 2 + 1) {
            for y in (-range / 2)..(range / 2 + 1) {
                for x in (-range / 2)..(range / 2 + 1) {
                    if active(&map, x, y, z) {
                        let count = get_count(&mut new_map, x, y, z);
                        update_state(&mut map, x, y, z, count == 2 || count == 3);
                    } else {
                        let count = get_count(&mut new_map, x, y, z);
                        update_state(&mut map, x, y, z, count == 3);
                    }
                }
            }
        }

        // println!("After {} cycles:", cyc + 1);
        // print_map(&map, range + (cyc + 1) * 2);
        // println!("");
    }

    // count active states
    let mut count = 0;
    for z in map.keys() {
        for y in map[z].keys() {
            for x in map[z][y].keys() {
                if map[z][y][x] {
                    count += 1;
                }
            }
        }
    }

    // somehow answer = count -1?
    format!("{}", count)
}

pub fn update_state2(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>, x: i64, y: i64, z: i64, w: i64, state: bool) {
    *map.entry(w).or_insert(HashMap::new()).entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(false) = state;
}

pub fn increment_count2(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, usize>>>>, x: i64, y: i64, z: i64, w: i64) {
    *map.entry(w).or_insert(HashMap::new()).entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(0) += 1;
}

pub fn get_count2(map: &mut HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, usize>>>>, x: i64, y: i64, z: i64, w: i64) -> usize {
    *map.entry(w).or_insert(HashMap::new()).entry(z).or_insert(HashMap::new()).entry(y).or_insert(HashMap::new()).entry(x).or_insert(0)
}

pub fn active2(map: &HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>, x: i64, y: i64, z: i64, w: i64) -> bool {
    if let Some(wmap) = map.get(&w) {
        if let Some(zmap) = wmap.get(&z) {
            if let Some(ymap) = zmap.get(&y) {
                if let Some(state) = ymap.get(&x) {
                    *state
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }   
    } else {
        false
    }
}

pub fn print_map2(map: &HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>, range: i64) {
    // println!("Called with range: {} and map {:?}", range, map);
    for w in (-range / 2)..(range / 2 + 1) {
        for z in (-range / 2)..(range / 2 + 1) {
            println!("z={}, w={}", z, w);
            for y in (-range / 2)..(range / 2 + 1) {
                for x in (-range / 2)..(range / 2 + 1) {
                    if active2(&map, x, y, z, w) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("");
        }
    }
}

pub fn part2(input: String) -> String {
    let mut map: HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>> = HashMap::new();

    let mut range = 0;
    // load in input
    input.lines().enumerate().for_each(|(j, l)| {
        range = l.len() as i64;
        let middle = (l.len() / 2) as i64;
        l.chars().enumerate().for_each(|(i, c)| {
            update_state2(&mut map, (i as i64) - middle, (j as i64) - middle, 0, 0, c == '#');
        });
    });

    // println!("Before any cycles:");
    // print_map2(&map, range);
    // println!("");

    let cycles = 6;
    
    for cyc in 0..cycles {
        let mut new_map: HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, usize>>>> = HashMap::new();
        // compute a neighbour count for every square
        for w in map.keys() {
            for l in &[-1, 0, 1] {
                for z in map[w].keys() {
                    for k in &[-1, 0, 1] {
                        for y in map[w][z].keys() {
                            for j in &[-1, 0, 1] {
                                for x in map[w][z][y].keys() {
                                    if !active2(&map, *x, *y, *z, *w) {
                                        continue;
                                    }
                                    for i in &[-1, 0, 1] {
                                        let (c_x, c_y, c_z, c_w) = (x+i, y + j, z + k, w + l);
                                        if c_x == *x && c_y == *y && c_z == *z && c_w == *w {
                                            continue;
                                        }
                                        increment_count2(&mut new_map, c_x, c_y, c_z, c_w);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for every count whether that spot will become active
        // for w in new_map.keys() {
        //     for z in new_map[w].keys() {
        //         for y in new_map[w][z].keys() {
        //             for x in new_map[w][z][y].keys() {
        //                 if active2(&map, *x, *y, *z, *w) {
        //                     // println!("Setting ({}, {}, {}) to {} ({} neighbours active)", x, y, z, new_map[z][y][x] == 2 || new_map[z][y][x] == 3,new_map[z][y][x]);
        //                     update_state2(&mut map, *x, *y, *z, *w, new_map[w][z][y][x] == 2 || new_map[w][z][y][x] == 3);
        //                 } else {
        //                     // println!("Setting ({}, {}, {}) to {} ({} neighbours active)", x, y, z, new_map[z][y][x] == 3, new_map[z][y][x]);
        //                     update_state2(&mut map, *x, *y, *z, *w, new_map[w][z][y][x] == 3);
        //                 }
        //             }
        //         }   
        //     }
        // }
        range += 2;
        
        for w in (-range / 2)..(range / 2 + 1) {
            for z in (-range / 2)..(range / 2 + 1) {
                for y in (-range / 2)..(range / 2 + 1) {
                    for x in (-range / 2)..(range / 2 + 1) {
                        if active2(&map, x, y, z, w) {
                            let count = get_count2(&mut new_map, x, y, z, w);
                            update_state2(&mut map, x, y, z, w, count == 2 || count == 3);
                        } else {
                            let count = get_count2(&mut new_map, x, y, z, w);
                            update_state2(&mut map, x, y, z, w, count == 3);
                        }
                    }
                }
            }
        }

        // println!("After {} cycles:", cyc + 1);
        // print_map2(&map, range);
        // println!("");
    }

    // count active states
    let mut count = 0;
    for w in map.keys(){
        for z in map[w].keys() {
            for y in map[w][z].keys() {
                for x in map[w][z][y].keys() {
                    if map[w][z][y][x] {
                        count += 1;
                    }
                }
            }
        }
    }

    // somehow answer = count -1?
    format!("{}", count)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    formatted_print("A", part1(read_file("input")));
    formatted_print("B", part2(read_file("input")));
}

pub fn read_file(file_name: &str) -> String {
    return std::fs::read_to_string(file_name).expect(format!("File {} not found", file_name).as_str());
}

fn formatted_print(part: &str, output: String) {
    println!("==================== Part {} ======================", part);
    for l in output.lines() {
        println!("| {:^46} |", l);
    }
    println!("==================================================");
}