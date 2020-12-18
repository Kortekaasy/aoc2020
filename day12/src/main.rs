// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    /*
    North:    0 degrees
    East:    90 degrees
    South:  180 degrees
    West:   270 degrees
    */
    let mut dir = 90;
    let (mut x, mut y) = (0, 0);
    input.lines().map(|l| l.split_at(1)).for_each(|(action, amount)| {
        match action {
            "N" => x += amount.parse::<i64>().expect("Expected numeric amount"),
            "E" => y += amount.parse::<i64>().expect("Expected numeric amount"),
            "S" => x -= amount.parse::<i64>().expect("Expected numeric amount"),
            "W" => y -= amount.parse::<i64>().expect("Expected numeric amount"),
            "L" => dir = (dir - amount.parse::<i64>().expect("Expected numeric amount") + 360) % 360,
            "R" => dir = (dir + amount.parse::<i64>().expect("Expected numeric amount") + 360) % 360,
            "F" => {
                match dir {
                    0 => x += amount.parse::<i64>().expect("Expected numeric amount"),
                    90 => y += amount.parse::<i64>().expect("Expected numeric amount"),
                    180 => x -= amount.parse::<i64>().expect("Expected numeric amount"),
                    270 => y -= amount.parse::<i64>().expect("Expected numeric amount"),
                    _ => panic!("Invalid direction!")
                }
            },
            _ => panic!("Invalid Action!")
        }
    });

    format!("{}", x.abs() + y.abs())
}

pub fn part2(input: String) -> String {
    let (mut ship_x, mut ship_y) = (0, 0);
    let (mut waypoint_x, mut waypoint_y) = (1, 10);
    input.lines().map(|l| l.split_at(1)).for_each(|(action, amount)| {
        match action {
            "N" => waypoint_x += amount.parse::<i64>().expect("Expected numeric amount"),
            "E" => waypoint_y += amount.parse::<i64>().expect("Expected numeric amount"),
            "S" => waypoint_x -= amount.parse::<i64>().expect("Expected numeric amount"),
            "W" => waypoint_y -= amount.parse::<i64>().expect("Expected numeric amount"),
            "L" =>  {
                match amount.parse::<i64>().expect("Expected numeric amount") {
                    90 => {
                        // waypoint_x, waypoint_y = waypoint_y, -waypoint_x
                        let tmp = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -tmp;
                    },
                    180 =>  {
                        // waypoint_x, waypoint_y = -waypoint_x, -waypoint_y
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    },
                    270 => {
                        // waypoint_x, waypoint_y = -waypoint_y, waypoint_x
                        let tmp = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = tmp;
                    },
                    _ => panic!("Invalid direction!"),
                }
            },
            "R" => {
                match amount.parse::<i64>().expect("Expected numeric amount") {
                    90 => {
                        // waypoint_x, waypoint_y = -waypoint_y, waypoint_x
                        let tmp = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = tmp;
                    },
                    180 => {
                        // waypoint_x, waypoint_y = -waypoint_x, -waypoint_y
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    },
                    270 => {
                        // waypoint_x, waypoint_y = waypoint_y, -waypoint_x
                        let tmp = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -tmp;
                    },
                    _ => panic!("Invalid direction!"),
                }
            },
            "F" => {
                let multiplier = amount.parse::<i64>().expect("Expected numeric amount");
                ship_x += multiplier * waypoint_x;
                ship_y += multiplier * waypoint_y;
            },
            _ => panic!("Invalid Action!")
        }
    });

    format!("{}", ship_x.abs() + ship_y.abs())
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