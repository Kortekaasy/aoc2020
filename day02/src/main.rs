// ========================= Challenge Logic ============================

pub struct Line {
    lower: usize,
    upper: usize,
    character: char,
    password: String
}

pub fn parse_line(input: &str) -> Line {
    let splitted: Vec<&str> = input.splitn(3, " ").collect();
    let range_split: Vec<&str> = splitted[0].splitn(2, "-").collect();

    Line {
        lower: range_split[0].parse::<usize>().expect("Could not parse lower range"),
        upper: range_split[1].parse::<usize>().expect("Could not parse upper range"),
        character: splitted[1].chars().next().expect("Could not parse character"),
        password: String::from(splitted[2])
    }
}

pub fn part1(input: String) -> String {
    let valid = input.lines()
        .map(|s| parse_line(s))
        .filter(|l| {
            let occurances = l.password.chars().filter(|c| *c == l.character).count();
            l.lower <= occurances && l.upper >= occurances
        })
        .count();
    format!("{}", valid)
}

pub fn part2(input: String) -> String {
    let valid = input.lines()
        .map(|s| parse_line(s))
        .filter(|l| {
            let chars = l.password.chars().collect::<Vec<char>>();
            (chars[l.lower - 1] == l.character) ^ (chars[l.upper - 1] == l.character)
        })
        .count();
    format!("{}", valid)
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