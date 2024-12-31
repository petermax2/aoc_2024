use std::{collections::HashMap, io::Read};

const SOLUTION_CAPACITY: usize = 100;
const QUEUE_CAPACITY: usize = 300;
const INPUT_CAPACITY: usize = 5120;

#[derive(Debug, Clone)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Sentence {
    pub operator: Operator,
    pub operand1: String,
    pub operand2: String,
    pub result: String,
    pub solved: bool,
}

#[derive(Debug)]
struct Puzzle {
    pub solution: HashMap<String, u8>,
    pub queue: Vec<Sentence>,
}

impl Puzzle {
    pub fn new() -> Self {
        Self {
            solution: HashMap::with_capacity(SOLUTION_CAPACITY),
            queue: Vec::with_capacity(QUEUE_CAPACITY),
        }
    }

    pub fn solve_part1(&self) -> u64 {
        let mut solution = self.solution.clone();
        let mut queue = self.queue.clone();

        while !queue.is_empty() {
            for sentence in queue.iter_mut() {
                let operands = (
                    solution.get(&sentence.operand1),
                    solution.get(&sentence.operand2),
                );
                if let (Some(val1), Some(val2)) = operands {
                    let result = match &sentence.operator {
                        Operator::And => *val1 == 1 && *val2 == 1,
                        Operator::Or => *val1 == 1 || *val2 == 1,
                        Operator::Xor => *val1 != *val2,
                    };
                    let result = match result {
                        true => 1,
                        false => 0,
                    };

                    solution.insert(sentence.result.clone(), result);
                    sentence.solved = true;
                }
            }
            queue.retain(|s| !s.solved);
        }

        solution
            .iter()
            .filter(|(key, val)| key.starts_with("z") && (**val) == 1)
            .map(|(key, _)| {
                let exponent = key[1..].parse::<u32>().unwrap();
                2_u64.pow(exponent)
            })
            .sum()
    }

    pub fn solve_part2(&self) -> u64 {
        todo!()
    }
}

impl From<String> for Puzzle {
    fn from(value: String) -> Self {
        let mut puzzle = Puzzle::new();

        let regex_solution = regex::Regex::new(r"(.*): (\d)").unwrap();
        let regex_and = regex::Regex::new(r"^(.*) AND (.*) -> (.*)$").unwrap();
        let regex_xor = regex::Regex::new(r"^(.*) XOR (.*) -> (.*)$").unwrap();
        let regex_or = regex::Regex::new(r"^(.*) OR (.*) -> (.*)$").unwrap();

        value.lines().for_each(|l| {
            if regex_solution.is_match(l) {
                for (_, [name, val]) in regex_solution.captures_iter(l).map(|c| c.extract()) {
                    let val = val.trim();
                    if val == "0" {
                        puzzle.solution.insert(name.to_string(), 0);
                    } else if val == "1" {
                        puzzle.solution.insert(name.to_string(), 1);
                    }
                }
            }
            if regex_and.is_match(l) {
                for (_, [op1, op2, res]) in regex_and.captures_iter(l).map(|c| c.extract()) {
                    puzzle.queue.push(Sentence {
                        operator: Operator::And,
                        operand1: op1.to_string(),
                        operand2: op2.to_string(),
                        result: res.to_string(),
                        solved: false,
                    })
                }
            }
            if regex_xor.is_match(l) {
                for (_, [op1, op2, res]) in regex_xor.captures_iter(l).map(|c| c.extract()) {
                    puzzle.queue.push(Sentence {
                        operator: Operator::Xor,
                        operand1: op1.to_string(),
                        operand2: op2.to_string(),
                        result: res.to_string(),
                        solved: false,
                    })
                }
            }
            if regex_or.is_match(l) {
                for (_, [op1, op2, res]) in regex_or.captures_iter(l).map(|c| c.extract()) {
                    puzzle.queue.push(Sentence {
                        operator: Operator::Or,
                        operand1: op1.to_string(),
                        operand2: op2.to_string(),
                        result: res.to_string(),
                        solved: false,
                    })
                }
            }
        });

        puzzle
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::with_capacity(INPUT_CAPACITY);

    if let Err(e) = stdin.read_to_string(&mut input) {
        eprintln!("Failed to read input data: {e}");
        return;
    }

    let puzzle = Puzzle::from(input);

    let solution1 = puzzle.solve_part1();

    println!("{solution1}");
}
