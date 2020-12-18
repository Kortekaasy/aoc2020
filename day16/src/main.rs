use std::collections::HashSet;
// ========================= Challenge Logic ============================

pub fn parse_rule(rule: &str) -> Vec<(i64, i64)> {
    let splitted: Vec<&str> = rule.split(": ").collect();
    splitted[1].split(" or ").map(|s| {
        let subsplit: Vec<&str> = s.split("-").collect();
        let lower = subsplit[0].parse::<i64>().expect("Expected to parse number");
        let upper = subsplit[1].parse::<i64>().expect("Expected to parse number");
        (lower, upper)
    }).collect()
}

pub fn check_rule(input: i64, rule: &Vec<(i64, i64)>) -> bool {
    rule.iter().fold(false, |acc, &x| acc || (input >= x.0 && input <= x.1))
}

pub fn check_rules(input: i64, rules: &Vec<Vec<(i64, i64)>>) -> bool {
    rules.iter().fold(false, |acc, x| acc || check_rule(input, x))
}

pub fn part1(input: String) -> String {
    let mut rules: Vec<Vec<(i64, i64)>> = Vec::new();

    // Split in rules, your ticket, nearby tickets
    let input_split: Vec<&str> = input.split("\n\n").collect();
    
    // parse rules
    for l in input_split[0].lines() {
        rules.push(parse_rule(l));
    }

    // check nearby tickets
    let mut sum = 0;
    for l in input_split[2].lines().skip(1) {
        sum += l.split(",").map(|x| x.parse::<i64>().expect("Expected to parse numeric input"))
            .filter(|&x| {
                let cond = !check_rules(x, &rules);
                // println!("{} passed rules? {}", x, !cond);
                cond
            }).sum::<i64>();
        // println!("");
    }

    format!("{}", sum)
}

pub fn part2(input: String) -> String {
    let mut rules: Vec<Vec<(i64, i64)>> = Vec::new();

    // Split in rules, your ticket, nearby tickets
    let input_split: Vec<&str> = input.split("\n\n").collect();
    
    // parse rules
    for l in input_split[0].lines() {
        rules.push(parse_rule(l));
    }

    // check validity of nearby tickets
    let mut valid_tickets: Vec<Vec<i64>> = Vec::new();
    for l in input_split[2].lines().skip(1) {
        let ticket: Vec<i64> = l.split(",").map(|x| x.parse::<i64>().expect("Expected to parse numeric input")).collect();
        if ticket.iter().filter(|&x| !check_rules(*x, &rules)).count() == 0 {
            valid_tickets.push(ticket);
        }
    }

    // base case, all rules are valid for every column
    let mut column_rules: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..rules.len() {
        column_rules.push((0..rules.len()).collect());
    }

    // println!("Valid tickets: {:?}", valid_tickets);

    for ticket in valid_tickets.iter() {
        for (i, &field) in ticket.iter().enumerate() {
            let mut set: HashSet<usize> = HashSet::new();
            for (j, rule) in rules.iter().enumerate() {
                if check_rule(field, rule) {
                    set.insert(j);
                }
            }
            // println!("Valid rules for {}: {:?}", i, set);
            let intersect = set.intersection(&column_rules[i]).map(|x| *x).collect();
            column_rules[i] = intersect;
        }
    }

    let mut used: HashSet<usize> = HashSet::new();
    let mut column_rules = column_rules.drain(..).enumerate().collect::<Vec<(usize, HashSet<usize>)>>();
    column_rules.sort_by_key(|s| s.1.len());

    let mut rule_column: Vec<usize> = vec![0; rules.len()];
    for (i, s) in column_rules.drain(..) {
        // print!("Ruleset for column {}: {:?}", i, s);
        for v in s.iter() {
            if !used.contains(v) {
                // println!(" - {}", v);
                rule_column[*v] = i;
                used.insert(*v);
                break;
            }
        }
    } 

    // println!("Rules: {:?}", rule_column);

    let my_ticket_splitted: Vec<&str> = input_split[1].split("\n").collect();
    let my_ticket: Vec<i64> = my_ticket_splitted[1].split(",").map(|x| x.parse::<i64>().expect("Expected to parse numeric input")).collect();

    // println!("My ticket: {:?}", my_ticket);
    let values = rule_column.iter().take(6).map(|&i| my_ticket[i]).collect::<Vec<i64>>();
    // println!("values: {:?}", values);
    let prod: i64 = values.iter().product();

    format!("{}", prod)
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