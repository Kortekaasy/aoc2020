// ========================= Challenge Logic ============================

const HEIGHT: usize = 323;
const WIDTH: usize = 31;

pub fn trees_for_slope(field: &[[bool; WIDTH]; HEIGHT], right: usize, down: usize) -> usize {   
    let mut trees = 0;
    let mut j = 0;

    for i in (0..HEIGHT).step_by(down) {
        if field[i][j] {
            trees += 1;
        }
        j = (j + right) % WIDTH;
    }

    trees
}

pub fn part1(input: String) -> String {

    let mut field: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    input.lines()
        .enumerate()
        .for_each(|(i, l)|
            l.chars().enumerate().for_each(|(j, c)| {
                field[i][j] = c == '#';
            })
        );
    
    let trees = trees_for_slope(&field, 3, 1);
    format!("{}", trees)
}

pub fn part2(input: String) -> String {
    let mut field: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    input.lines()
        .enumerate()
        .for_each(|(i, l)|
            l.chars().enumerate().for_each(|(j, c)| {
                field[i][j] = c == '#';
            })
        );
    
    let trees_1_1 = trees_for_slope(&field, 1, 1);
    let trees_3_1 = trees_for_slope(&field, 3, 1);
    let trees_5_1 = trees_for_slope(&field, 5, 1);
    let trees_7_1 = trees_for_slope(&field, 7, 1);
    let trees_1_2 = trees_for_slope(&field, 1, 2);
    
    let trees = trees_1_1 * trees_3_1 * trees_5_1 * trees_7_1 * trees_1_2;
    format!("{}", trees)
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