use std::io;

use itertools::Itertools;

fn main() {
    let mut line = String::new();
    let mut save_reports: u64 = 0;
    let mut save_reports2: u64 = 0;

    // read and parse input
    let stdin = io::stdin();
    while let Ok(_) = stdin.read_line(&mut line) {
        if line.trim().is_empty() {
            break;
        }

        let levels: Vec<i64> = line
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        // part 1
        let ascending = levels.windows(2).all(|w| w[0] < w[1]);
        let descending = levels.windows(2).all(|w| w[0] > w[1]);
        let correct_diff = levels.windows(2).all(|w| {
            let delta = (w[0] - w[1]).abs();
            delta >= 1 && delta <= 3
        });

        if correct_diff && (ascending || descending) {
            save_reports += 1;
        }

        // part 2
        let dampened_save = levels.iter().combinations(levels.len() - 1).any(|subset| {
            let ascending = subset.windows(2).all(|w| w[0] < w[1]);
            let descending = subset.windows(2).all(|w| w[0] > w[1]);
            let correct_diff = subset.windows(2).all(|w| {
                let delta = (w[0] - w[1]).abs();
                delta >= 1 && delta <= 3
            });

            correct_diff && (ascending || descending)
        });

        if dampened_save {
            save_reports2 += 1;
        }

        line.clear();
    }

    println!("{save_reports}");
    println!("{save_reports2}");
}
