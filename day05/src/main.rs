// ========================= Challenge Logic ============================

pub fn parse_line(input: &str) -> (usize, usize) {
    let (mut row, mut col) = (0, 0);
    
    input.chars().take(7).enumerate().for_each(|(i, val)| {
        let delta = 1 << (7 - i - 1);
        match val {
            'F' => (),
            'B' => row += delta,
            _ => panic!("Wrong input!")
        }
    });

    input.chars().skip(7).enumerate().for_each(|(i, val)| {
        let delta = 1 << (3 - i - 1);
        match val {
            'L' => (),
            'R' => col += delta,
            _ => panic!("Wrong input!")
        }
    });
    
    return (row, col);
}

pub fn part1(input: String) -> String {
    let max = input.lines().map(parse_line).map(|v| v.0 * 8 + v.1).max();
    format!("{}", max.expect("Could not calculate max!"))
}

pub fn part2(input: String) -> String {
    let mut ids = input.lines().map(parse_line).map(|v| v.0 * 8 + v.1).collect::<Vec<usize>>();
    
    ids.sort();
    
    let mut prev_id = 0;
    for id in ids {
        if id - prev_id != 1 && prev_id != 0 {
            return format!("{}", prev_id + 1);
        }
        prev_id = id;
    }

    format!("Could not find ID!")
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