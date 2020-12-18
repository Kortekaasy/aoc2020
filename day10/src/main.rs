use std::collections::HashMap;
// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let mut nums: Vec<i64> = input.lines().map(|v| v.parse::<i64>().expect("Expected to parse number!")).collect();
    nums.sort();

    nums.insert(0, 0);
    nums.push(nums.iter().max().expect("Expected to find max") + 3);
    
    let diffs: Vec<i64> = nums.iter().take(nums.len() - 1).zip(nums.iter().skip(1)).map(|(a, b)| b - a).collect();

    let ones = diffs.iter().filter(|&v| *v == 1).count();
    let threes = diffs.iter().filter(|&v| *v == 3).count();

    format!("{}", ones * threes)
}

pub fn part2(input: String) -> String {
    let mut nums: Vec<i64> = input.lines().map(|v| v.parse::<i64>().expect("Expected to parse number!")).collect();
    nums.sort();

    nums.insert(0, 0);
    nums.push(nums.iter().max().expect("Expected to find max") + 3);

    let mut map: HashMap<i64, i128> = HashMap::new();
    map.insert(0, 1);

    for i in 1..nums.len() {
        // find adapters below that are suitable
        let mut sum = 0;
        for j in (0..i).rev().take(3) {
            // if distance is less or equal to 3, we have a valid adapter
            if nums[i] - nums[j] <= 3 {
                sum += map.get(&nums[j]).expect(format!("{} should already be in the map!", nums[j]).as_str());
            }
        }
        map.insert(nums[i], sum);
    }

    format!("{}", map.get(nums.iter().max().expect("Expected to find max")).expect("Expected max in map"))
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