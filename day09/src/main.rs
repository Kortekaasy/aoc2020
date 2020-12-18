// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let preamble_size = 25;
    let nums: Vec<i64> = input.lines().map(|v| v.parse::<i64>().expect("Expected number to parse")).collect();

    for i in preamble_size..nums.len() {
        let mut cond = false;
        for j in 1..(preamble_size + 1) {
            for k in 1..(preamble_size + 1) {
                if j == k {
                    continue;
                }
                cond |= nums[i-j] + nums[i-k] == nums[i];
            }
        }
        if !cond {
            return format!("{}", nums[i]);
        }
    }

    format!("All input is valid!")
}

pub fn part2(input: String) -> String {
    let target = /*36845998;*/ part1(input.clone()).parse::<i64>().expect("Expected to find number here!");
    let nums: Vec<i64> = input.lines().map(|v| v.parse::<i64>().expect("Expected number to parse")).collect();

    for i in 0..nums.len() {
        let mut sum = nums[i];

        let mut j = i + 1;
        while sum < target && j < nums.len() {
            sum += nums[j];
            j += 1;
        }

        if sum == target {
            let contig_range: Vec<i64> = nums.iter().skip(i).take(j-i).map(|v| *v).collect();
            let min = contig_range.iter().min().expect("Expected to find minimum");
            let max = contig_range.iter().max().expect("Expected to find maximum");
            return format!("{} + {} = {}", min, max, min + max);
        }
    }

    format!("Couldn not find range")
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
