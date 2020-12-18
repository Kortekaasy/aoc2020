// ========================= Challenge Logic ============================

pub fn part1(input: String) -> String {
    let eta = input.lines().nth(0).expect("Expected first line to be available")
                    .parse::<i128>().expect("Exped to parse bus ID");
    let mut bustimes = input.lines().nth(1).expect("Expected second line to be available")
                    .split(",")
                    .map(|x| x.parse::<i128>())
                    .filter(|x| x.is_ok())
                    .map(|bus_id| {
                        let unwrapped = bus_id.unwrap();
                        (unwrapped, unwrapped - (eta % unwrapped))
                    })
                    .collect::<Vec<(i128, i128)>>();
    bustimes.sort_unstable_by_key(|a| a.1);
    let (bus_id, time) = bustimes[0];
    format!("{}", bus_id * time)
}

pub fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
pub fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
pub fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}

pub fn part2(input: String) -> String {
    let bustimes: Vec<(i128, i128)> = input.lines().nth(1).expect("Expected second line to be available")
                    .split(",")
                    .map(|x| x.parse::<i128>())
                    .enumerate()
                    .filter(|(_i, id)| id.is_ok())
                    .map(|(i, id)| {
                        let expected = id.expect("Expected parsed number");
                        (expected - i as i128, expected)
                    }).collect();
    let residues = bustimes.iter().map(|&(i, _id)| i).collect::<Vec<i128>>();
    let moduli = bustimes.iter().map(|&(_i, id)| id).collect::<Vec<i128>>();
    
    format!("{}", chinese_remainder(&residues, &moduli).expect("Expected to find solution"))
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

/*
value of t = 1068781
value of x7 = 56252
value of x1 = 82214
value of x6 = 34477
value of x4 = 18115
value of x0 = 152683
*/

/*
x0: 201
x2: 263
t: 3417
x3: 180
*/