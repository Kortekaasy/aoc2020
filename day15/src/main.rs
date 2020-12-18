use std::collections::HashMap;
// ========================= Challenge Logic ============================

pub fn find_number_of_turn(input_numbers: &Vec<usize>, target: usize) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();

    for (i, &num) in input_numbers.iter().enumerate().take(input_numbers.len() - 1) {
        mem.insert(num, i + 1);
    }

    let mut turn = input_numbers.len();
    let mut last_spoken = input_numbers[turn - 1];

    while turn < target {
        if mem.contains_key(&last_spoken) {
            let echo = last_spoken;
            last_spoken = turn - mem[&last_spoken];
            mem.insert(echo, turn);
        } else {
            let echo = last_spoken;
            last_spoken = 0;
            mem.insert(echo, turn);
        }
        
        turn += 1;
    }

    last_spoken
}

pub fn part1(input: String) -> String {
    let input_numbers: Vec<usize> = input.split(",").map(|n| n.parse::<usize>().expect("Expected to parse number")).collect();
    format!("{}", find_number_of_turn(&input_numbers, 2020))
}

pub fn part2(input: String) -> String {
    let input_numbers: Vec<usize> = input.split(",").map(|n| n.parse::<usize>().expect("Expected to parse number")).collect();
    format!("{}", find_number_of_turn(&input_numbers, 30_000_000))
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

/*
./target/release/day15  3,29s user 0,05s system 99% cpu 3,351 total
./target/release/day15  3,25s user 0,07s system 99% cpu 3,323 total
./target/release/day15  3,30s user 0,04s system 99% cpu 3,346 total
./target/release/day15  3,32s user 0,05s system 99% cpu 3,372 total
./target/release/day15  3,60s user 0,06s system 99% cpu 3,661 total
./target/release/day15  3,78s user 0,07s system 99% cpu 3,861 total
./target/release/day15  3,73s user 0,05s system 99% cpu 3,785 total
./target/release/day15  3,36s user 0,05s system 99% cpu 3,424 total
./target/release/day15  3,40s user 0,05s system 99% cpu 3,457 total
./target/release/day15  3,40s user 0,05s system 99% cpu 3,456 total
*/