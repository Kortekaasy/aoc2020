mod part1;
mod part2;
// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    format!("{}", part1::parse_and_execute(input))
}

pub fn part2(input: String) -> String {
    format!("{}", part2::parse_and_execute(input))
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