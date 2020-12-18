use std::collections::HashMap;
// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let mut field: HashMap<&str, &str> = [
        ("byr", ""), ("iyr", ""), ("eyr", ""), ("hgt", ""), ("hcl", ""), ("ecl", ""), ("pid", ""),
    ].iter().cloned().collect();

    let mut valid = 0;

    for l in input.lines() {
        if l.is_empty() {
            if field.values().filter(|&v| !v.is_empty()).count() == 7 {
                valid += 1;
            }
            field = [
                ("byr", ""), ("iyr", ""), ("eyr", ""), ("hgt", ""), ("hcl", ""), ("ecl", ""), ("pid", ""),
            ].iter().cloned().collect();
        } else {
            l.split(" ").for_each(|s| {
                let key = s.split_at(3).0;
                let value = s.split_at(4).1;
                if field.contains_key(key) {
                    *field.entry(key).or_default() = value;
                }
            });
        }
    }

    format!("{}", valid)
}

pub fn validate(input: (&str, &str)) -> bool {
    match input.0 {
        "byr" => {
            let i = input.1.parse::<i64>().unwrap_or(-1);
            i >= 1920 && i <= 2002
        },
        "iyr" => {
            let i = input.1.parse::<i64>().unwrap_or(-1);
            i >= 2010 && i <= 2020
        },
        "eyr" => {
            let i = input.1.parse::<i64>().unwrap_or(-1);
            i >= 2020 && i <= 2030
        },
        "hgt" =>  {
            let (hgh, unit) = input.1.split_at(input.1.len() - 2);
            match unit {
                "cm" => {
                    let i = hgh.parse::<i64>().unwrap_or(-1);
                    i >= 150 && i <= 193
                }
                "in" => {
                    let i = hgh.parse::<i64>().unwrap_or(-1);
                    i >= 59 && i <= 76
                },
                _ => false
            }
        },
        "hcl" => {
            let (hash, val) = input.1.split_at(1);
            hash == "#" && val.len() == 6 && i64::from_str_radix(val, 16).is_ok()
        },
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input.1)
        },
        "pid" => {
            let i = input.1.parse::<i64>().unwrap_or(-1);
            input.1.len() == 9 && i != -1
        },
        _ => false
    }
}

pub fn part2(input: String) -> String {
    let mut field: HashMap<&str, &str> = HashMap::new();

    let mut valid = 0;

    for l in input.lines() {
        if l.is_empty() {
            if field.iter().filter(|&v| validate((v.0, v.1))).count() == 7 {
                valid += 1;
            }
            field.clear();
        } else {
            l.split(" ").for_each(|s| {
                let key = s.split_at(3).0;
                let value = s.split_at(4).1;
                field.insert(key, value);
            });
        }
    }

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