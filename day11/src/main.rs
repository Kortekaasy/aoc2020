// ========================= Challenge Logic ============================
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Spot {
    Floor,
    Empty,
    Occupied
}

#[allow(dead_code)]
pub fn print_board(board: &Vec<Vec<Spot>>) {
    for r in board {
        for cell in r {
            match *cell {
                Spot::Floor => print!("."),
                Spot::Empty => print!("L"),
                Spot::Occupied => print!("#"),
            }
        }
        println!("");
    }
}

pub fn compare_boards(a: &Vec<Vec<Spot>>, b: &Vec<Vec<Spot>>) -> bool {
    let mut res = true;

    a.iter().zip(b.iter()).for_each(|(ra, rb)| {
        ra.iter().zip(rb.iter()).for_each(|(&sa, &sb)| {
            res &= sa == sb;
        })
    });

    res
}

pub fn count_occupied(board: &Vec<Vec<Spot>>) -> i64 {
    let mut res = 0;

    board.iter().for_each(|r| {
        r.iter().for_each(|&sa| {
            if sa == Spot::Occupied {
                res += 1;
            }
        })
    });

    res
}

pub fn compute_next_seating(board: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let mut res: Vec<Vec<Spot>> = Vec::new();

    for j in 0..board.len() {
        let mut row: Vec<Spot> = Vec::new();
        for i in 0..board[0].len() {
            let mut occ_sum = 0;
            for y in (-1)..(2 as i64) {
                for x in (-1)..(2 as i64) {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if let Some(v) = board.get((j as i64 + y) as usize) {
                        if let Some(s) = v.get((i as i64 + x) as usize) {
                            match s {
                                Spot::Floor => (),
                                Spot::Empty => (),
                                Spot::Occupied => occ_sum += 1,
                            }
                        }
                    }
                }
            }

            if occ_sum == 0 && board[j][i] == Spot::Empty {
                row.push(Spot::Occupied);
            } else if occ_sum >= 4 && board[j][i] == Spot::Occupied {
                row.push(Spot::Empty);
            } else {
                row.push(board[j][i]);
            }
        }
        res.push(row);
    }

    return res;
}

pub fn part1(input: String) -> String {
    // create board
    let mut board: Vec<Vec<Spot>> = Vec::new();
    
    // load in board from input
    input.lines().for_each(|l| {
        board.push(l.chars().map(|c| match c {
            'L' => Spot::Empty,
            '.' => Spot::Floor,
            _ => panic!("Found unexpected character!")
        }).collect());
    });

    // print_board(&board);
    // println!("");

    loop {
        let new_board = compute_next_seating(&board);

        // print_board(&new_board);
        // println!("");
        
        if compare_boards(&board, &new_board) {
            board = new_board;
            break;
        } else {
            board = new_board;
        }
    }

    format!("{}", count_occupied(&board))
}

pub fn compute_next_seating_2(board: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let mut res: Vec<Vec<Spot>> = Vec::new();

    for j in 0..board.len() {
        let mut row: Vec<Spot> = Vec::new();
        for i in 0..board[0].len() {
            let mut occ_sum = 0;
            // left
            for x in (0..i).rev() { 
                match board[j][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // right
            for x in (i+1)..board[0].len() { 
                match board[j][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // top
            for y in (0..j).rev() { 
                match board[y][i] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // bottom
            for y in (j+1)..board.len() { 
                match board[y][i] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // top left
            for (y, x) in (0..j).rev().zip((0..i).rev()) { 
                match board[y][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // top right
            for (y, x) in (0..j).rev().zip((i+1)..board[0].len()) { 
                match board[y][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // bottom left
            for (y, x) in ((j+1)..board.len()).zip((0..i).rev()) { 
                match board[y][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            // bottom right
            for (y, x) in ((j+1)..board.len()).zip((i+1)..board[0].len()) { 
                match board[y][x] {
                    Spot::Occupied => {
                        occ_sum += 1;
                        break;
                    },
                    Spot::Empty => break,
                    Spot::Floor => ()
                }
            }

            if occ_sum == 0 && board[j][i] == Spot::Empty {
                row.push(Spot::Occupied);
            } else if occ_sum >= 5 && board[j][i] == Spot::Occupied {
                row.push(Spot::Empty);
            } else {
                row.push(board[j][i]);
            }
        }
        res.push(row);
    }

    return res;
}

pub fn part2(input: String) -> String {
    // create board
    let mut board: Vec<Vec<Spot>> = Vec::new();
    
    // load in board from input
    input.lines().for_each(|l| {
        board.push(l.chars().map(|c| match c {
            'L' => Spot::Empty,
            '.' => Spot::Floor,
            _ => panic!("Found unexpected character!")
        }).collect());
    });

    // print_board(&board);
    // println!("");

    loop {
        let new_board = compute_next_seating_2(&board);

        // print_board(&new_board);
        // println!("");
        
        if compare_boards(&board, &new_board) {
            board = new_board;
            break;
        } else {
            board = new_board;
        }
    }

    format!("{}", count_occupied(&board))
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
