use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, combinator::value,
    multi::separated_list1, sequence::tuple, IResult,
};

const INPUT_CAPACITY: usize = 5120;

#[derive(Debug, Clone)]
enum Operator {
    And,
    Or,
    Xor,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value.trim() {
            "AND" => Operator::And,
            "XOR" => Operator::Xor,
            "OR" => Operator::Or,
            op => panic!("Unsupported operator: {op}"),
        }
    }
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

    pub fn solve_part2(&self) -> String {
        let result: Vec<String> = Vec::new();

        // TODO finish part 2

        assert!(result.len() == 8);
        result.join(",")
    }

    fn parse_solution(input: &str) -> IResult<&str, (String, u8)> {
        let (input, (name, _, digit)) = tuple((
            alphanumeric1,
            tag(": "),
            alt((value(0_u8, tag("0")), value(1_u8, tag("1")))),
        ))(input)?;

        Ok((input, (name.to_string(), digit)))
    }

    fn parse_sentence(input: &str) -> IResult<&str, Sentence> {
        let (input, (operand1, operator, operand2, _, result)) = tuple((
            alphanumeric1,
            alt((tag(" AND "), tag(" XOR "), tag(" OR "))),
            alphanumeric1,
            tag(" -> "),
            alphanumeric1,
        ))(input)?;

        Ok((
            input,
            Sentence {
                operator: operator.into(),
                operand1: operand1.to_string(),
                operand2: operand2.to_string(),
                result: result.to_string(),
                solved: false,
            },
        ))
    }
}

impl From<String> for Puzzle {
    fn from(value: String) -> Self {
        let (_, (solution, _, queue)) = tuple((
            separated_list1(tag("\n"), Puzzle::parse_solution),
            tag("\n\n"),
            separated_list1(tag("\n"), Puzzle::parse_sentence),
        ))(&value)
        .unwrap();

        Puzzle {
            solution: solution.into_iter().collect(),
            queue,
        }
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

    let solution2 = puzzle.solve_part2();
    println!("{solution2}");
}
