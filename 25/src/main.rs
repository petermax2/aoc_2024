use rayon::prelude::*;
use std::io::Read;

const INPUT_CAPACITY: usize = 20480;
const VECTOR_CAPACITY: usize = 500;

type ReducedPuzzleInput = (i32, i32, i32, i32, i32);

const EMPTY_PUZZLE_INPUT: ReducedPuzzleInput = (0, 0, 0, 0, 0);
const HEIGHT_LIMIT: i32 = 6;

/// Returns keys and locks
fn parse_input(input: &str) -> (Vec<ReducedPuzzleInput>, Vec<ReducedPuzzleInput>) {
    let mut keys: Vec<ReducedPuzzleInput> = Vec::with_capacity(VECTOR_CAPACITY);
    let mut locks: Vec<ReducedPuzzleInput> = Vec::with_capacity(VECTOR_CAPACITY);
    let mut key_mode: Option<bool> = None;
    let mut pin_heights: ReducedPuzzleInput = EMPTY_PUZZLE_INPUT;

    input.lines().for_each(|l| {
        if l.trim().is_empty() && key_mode.is_some() {
            let is_key = key_mode.unwrap();
            if is_key {
                keys.push(pin_heights);
            } else {
                pin_heights.0 -= 1;
                pin_heights.1 -= 1;
                pin_heights.2 -= 1;
                pin_heights.3 -= 1;
                pin_heights.4 -= 1;
                locks.push(pin_heights);
            }
            key_mode = None;
            pin_heights = EMPTY_PUZZLE_INPUT;
        }

        match key_mode {
            Some(_) => {
                let values: Vec<i32> = l
                    .trim()
                    .as_bytes()
                    .iter()
                    .map(|x| if *x == b'#' { 1_i32 } else { 0_i32 })
                    .collect();

                pin_heights.0 += values[0];
                pin_heights.1 += values[1];
                pin_heights.2 += values[2];
                pin_heights.3 += values[3];
                pin_heights.4 += values[4];
            }
            None => {
                key_mode = match l.trim() {
                    "#####" => Some(true),
                    "....." => Some(false),
                    _ => None,
                };
            }
        }
    });

    if let Some(is_key) = key_mode {
        if is_key {
            keys.push(pin_heights);
        } else {
            pin_heights.0 -= 1;
            pin_heights.1 -= 1;
            pin_heights.2 -= 1;
            pin_heights.3 -= 1;
            pin_heights.4 -= 1;
            locks.push(pin_heights);
        }
    }

    (keys, locks)
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::with_capacity(INPUT_CAPACITY);

    if let Err(e) = stdin.read_to_string(&mut input) {
        eprintln!("Failed to read input data: {e}");
        return;
    }

    let (keys, locks) = parse_input(&input);

    // solve part 1 - let's speed things by uing rayon crate for parallelization
    let part1: usize = keys
        .par_iter()
        .map(|k| {
            locks
                .par_iter()
                .filter(|l| {
                    k.0 + l.0 < HEIGHT_LIMIT
                        && k.1 + l.1 < HEIGHT_LIMIT
                        && k.2 + l.2 < HEIGHT_LIMIT
                        && k.3 + l.3 < HEIGHT_LIMIT
                        && k.4 + l.4 < HEIGHT_LIMIT
                })
                .count()
        })
        .sum();
    println!("{part1}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_lock() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....";
        let (locks, _) = parse_input(input);
        let lock = locks[0];
        assert_eq!(lock, (0, 5, 3, 4, 3));
    }

    #[test]
    fn example_key() {
        let input = ".....
#....
#....
#...#
#.#.#
#.###
#####";

        let (_, keys) = parse_input(input);
        let key = keys[0];
        assert_eq!(key, (5, 0, 2, 1, 3));
    }
}
