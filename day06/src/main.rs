// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let mut form: [bool; 26] = [false; 26];

    let mut sum = 0;

    for l in input.lines() {
        if l.is_empty() {
            sum += form.iter().filter(|v| **v).count();
            form = [false; 26];
        } else {
            l.chars().for_each(|c| form[(c as usize) - ('a' as usize)] = true);
        }
    }

    format!("{}", sum)
}

pub fn part2(input: String) -> String {
    let mut form: [i64; 26] = [0; 26];

    let mut sum = 0;
    let mut group_members = 0;

    for l in input.lines() {
        if l.is_empty() {
            sum += form.iter().filter(|v| **v == group_members).count();
            group_members = 0;
            form = [0; 26];
        } else {
            l.chars().for_each(|c| form[(c as usize) - ('a' as usize)] += 1);
            group_members += 1;
        }
    }

    format!("{}", sum)
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