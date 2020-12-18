use std::collections::HashMap;
// ========================= Challenge Logic ============================
pub fn apply_mask(mut input: usize, mask: &str) -> usize {
    mask.chars().rev().enumerate().for_each(|(i, c)| {
        let bit: usize = 1 << i; 
        match c {
            'X' => (),
            '0' => {
                if (input & bit) >> i == 1 {
                    input ^= bit;
                }
            },
            '1' => {
                if (input & bit) >> i == 0 {
                    input ^= bit;
                }
            },
            _ => panic!(format!("Invalid char {}", c)),
        }
    });

    input
}


pub fn part1(input: String) -> String {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    input.lines().for_each(|l| {
        let splitted = l.split(" = ").collect::<Vec<&str>>();
        if splitted[0].starts_with("mask") {
            mask = splitted[1];
        } else {
            let latter = splitted[0].split_at(4).1;
            let addr = (latter.split_at(latter.len() - 1).0).parse::<usize>().expect("Expected to find numerical address");
            let val = splitted[1].parse::<usize>().expect("Expected numerical value");

            *mem.entry(addr).or_insert(0) = apply_mask(val, mask);
        }
    });

    format!("{}", mem.values().sum::<usize>())
}

pub fn apply_mask_2(input: usize, mask: &str) -> Vec<usize> {
    let mut vals: Vec<usize> = vec![input];
    mask.chars().rev().enumerate().for_each(|(i, c)| {
        let bit: usize = 1 << i; 
        match c {
            'X' => {
                let mut new_vals: Vec<usize> = Vec::new();
                for v in vals.iter() {
                    new_vals.push(*v);
                    new_vals.push(*v ^ bit);
                }
                vals = new_vals;
            },
            '0' => (),
            '1' => {
                for x in vals.iter_mut() {
                    *x |= bit;
                }
            },
            _ => panic!(format!("Invalid char {}", c)),
        }
    });

    vals
}

pub fn part2(input: String) -> String {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    input.lines().for_each(|l| {
        let splitted = l.split(" = ").collect::<Vec<&str>>();
        if splitted[0].starts_with("mask") {
            mask = splitted[1];
        } else {
            let latter = splitted[0].split_at(4).1;
            let addr = (latter.split_at(latter.len() - 1).0).parse::<usize>().expect("Expected to find numerical address");
            let val = splitted[1].parse::<usize>().expect("Expected numerical value");

            let addresses = apply_mask_2(addr, mask);
            for a in addresses {
                *mem.entry(a).or_insert(0) = val;
            }
        }
    });

    format!("{}", mem.values().sum::<usize>())
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