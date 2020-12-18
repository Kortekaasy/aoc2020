// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let parsed = input.lines().map(|l| l.parse::<i64>().expect("Failed to parse")).collect::<Vec<i64>>();
    let mut set: [bool; 2020] = [false; 2020];

    parsed.iter().for_each(|n| set[*n as usize] = true);
    
    for i in parsed.iter() {
        if set[(2020 - i) as usize] {
            return format!("{}", i * (2020 - i));
        }
    }

    format!("Not Found!")
}

pub fn part2(input: String) -> String {
    let parsed = input.lines().map(|l| l.parse::<i64>().expect("Failed to parse")).collect::<Vec<i64>>();
    let mut set: [bool; 2020] = [false; 2020];

    parsed.iter().for_each(|n| set[*n as usize] = true);
    
    for i in parsed.iter() {
        for j in parsed.iter() {
            if i == j {
                continue;
            } else if (2020 - i - j) > 0 && set[(2020 - i - j) as usize] {
                return format!("{}", i * j * (2020 - i - j));
            }
        }
    }

    format!("Not Found!")
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