use std::collections::HashSet;
// ========================= Challenge Logic ============================

#[derive(Clone, Debug)]
pub enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

pub struct HandHeld {
    pc: usize,
    instructions: Vec<Instruction>,
    accumulator: i64,
}

impl HandHeld {
    #[allow(dead_code)]
    pub fn new() -> HandHeld {
        HandHeld {
            pc: 0,
            instructions: Vec::new(),
            accumulator: 0
        }
    }

    pub fn with_instructions(instructions: Vec<Instruction>) -> HandHeld {
        HandHeld {
            pc: 0,
            instructions: instructions,
            accumulator: 0
        }
    }

    pub fn step(&mut self) -> usize {
        let inc = match self.instructions[self.pc] {
            Instruction::NOP(_) => 1,
            Instruction::ACC(a) => {
                self.accumulator += a;
                1
            },
            Instruction::JMP(j) => j,
        };

        // println!("Executing {:?} - increment: {}", self.instructions[self.pc], inc);

        self.pc = (self.pc as i64 + inc) as usize;
        return self.pc;
    }

    pub fn run(&mut self) -> bool {
        
        let mut instructions_had: HashSet<usize> = HashSet::new();
        let mut next_instruction = 0;
        
        loop {
            if next_instruction == self.instructions.len() {
                return true;
            } else if instructions_had.contains(&next_instruction) {
                return false;
            } else {
                instructions_had.insert(next_instruction);
                next_instruction = self.step();
            }
        }
    }
}

pub fn parse_line(input: &str) -> Instruction {
    let splitted = input.splitn(2, " ").collect::<Vec<&str>>();
    match (splitted[0], splitted[1]) {
        ("nop", val) => Instruction::NOP(val.parse::<i64>().expect("Expected to parse a number")),
        ("acc", val) => Instruction::ACC(val.parse::<i64>().expect("Expected to parse a number")),
        ("jmp", val) => Instruction::JMP(val.parse::<i64>().expect("Expected to parse a number")),
        (op, val) => panic!("Found unexpected instruction: \"{}: {}\"", op, val),
    }
}

pub fn part1(input: String) -> String {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();
    let mut hh = HandHeld::with_instructions(instructions);

    hh.run();
    
    format!("{}", hh.accumulator)
}

pub fn part2(input: String) -> String {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();

    let mut nop_or_jump: Vec<(usize, Instruction)> = instructions.iter().enumerate().filter(|(_i, v)| {
        match v {
            Instruction::NOP(_) => true,
            Instruction::ACC(_) => false,
            Instruction::JMP(_) => true,
        }
    }).map(|(i, v)| {
        match v {
            Instruction::NOP(x) => (i, Instruction::JMP(*x)),
            Instruction::JMP(x) => (i, Instruction::NOP(*x)),
            Instruction::ACC(_) => panic!("Should not occur because of filter")
        }
    }).collect();

    for (i, v) in nop_or_jump.drain(..) {
        let mut patched_instructions = instructions.clone();
        patched_instructions[i] = v;
        
        let mut hh = HandHeld::with_instructions(patched_instructions);
        if hh.run() {
            return format!("{}", hh.accumulator);
        }
    }
    
    format!("Could not find Instruction to patch")
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
