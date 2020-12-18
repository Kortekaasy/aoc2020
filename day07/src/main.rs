use std::collections::HashMap;
use std::collections::HashSet;
// ========================= Challenge Logic ============================

// pub fn part1(input: String) -> String {
//     let mut map: HashMap<&str, Vec<(i64, &str)>> = HashMap::new();
//     input.lines().for_each(|l| {
//         let splitted: Vec<&str> = l.splitn(2, " bags contain ").collect();
        
//         let k = splitted[0];
//         let v = splitted[1].split(", ").filter(|s|!s.contains("no other bags.")).map(|s|{
//             let splitted_part: Vec<&str> = s.splitn(2, " ").collect();
//             let qty = splitted_part[0].parse::<i64>().expect("Expected numeric bag value");
//             let col = splitted_part[1].rsplitn(2, " ").nth(1).expect("Expected bag colour");
//             return (qty, col);
//         }).collect::<Vec<(i64, &str)>>();

//         map.insert(k, v);
//     });
//     println!("{:?}", map);
//     format!("Unimplemented!")
// }

pub fn part1(input: String) -> String {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let splitted: Vec<&str> = l.splitn(2, " bags contain ").collect();
        
        let v = splitted[0];
        splitted[1].split(", ").filter(|s|!s.contains("no other bags.")).for_each(|s|{
            let splitted_part: Vec<&str> = s.splitn(2, " ").collect();
            let col = splitted_part[1].rsplitn(2, " ").nth(1).expect("Expected bag colour");
            map.entry(col).or_insert(HashSet::new()).insert(v);
        });
    });
    
    let mut bags: HashSet<&str> = HashSet::new();
    let mut to_go: Vec<&str> = vec!["shiny gold"];

    // println!("{:?}", map);

    loop {
        let cur = to_go.pop().expect("Should have an element");
        let cur_cols = map.get(cur);
        if let Some(cols) = cur_cols {
            cols.iter().for_each(|c| to_go.push(c));
            bags = bags.union(cols).map(|&v| v).collect();
            // println!("bags: {:?}; togo: {:?}", bags, to_go);
        }
        
        if to_go.is_empty() {
            break;
        }
    }

    format!("{}", bags.len())
}

pub fn get_nr_of_bags(key: &str, map: &HashMap<&str, Vec<(i64, &str)>>) -> i64 {
    if let Some(v) = map.get(key) {
        v.iter().map(|(i, k)| i * get_nr_of_bags(k, map)).sum::<i64>() + 1
    } else {
        0
    }
}

pub fn part2(input: String) -> String {
    let mut map: HashMap<&str, Vec<(i64, &str)>> = HashMap::new();
    input.lines().for_each(|l| {
        let splitted: Vec<&str> = l.splitn(2, " bags contain ").collect();
        
        let k = splitted[0];
        let v = splitted[1].split(", ").filter(|s|!s.contains("no other bags.")).map(|s|{
            let splitted_part: Vec<&str> = s.splitn(2, " ").collect();
            let qty = splitted_part[0].parse::<i64>().expect("Expected numeric bag value");
            let col = splitted_part[1].rsplitn(2, " ").nth(1).expect("Expected bag colour");
            return (qty, col);
        }).collect::<Vec<(i64, &str)>>();

        map.insert(k, v);
    });

    format!("{}", get_nr_of_bags("shiny gold", &map) - 1)
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