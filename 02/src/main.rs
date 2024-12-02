use std::io;

fn main() {
    let mut line = String::new();
    let mut save_reports: u64 = 0;

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

        let ascending = levels.windows(2).all(|w| w[0] < w[1]);
        let descending = levels.windows(2).all(|w| w[0] > w[1]);
        let correct_diff = levels.windows(2).all(|w| {
            let delta = (w[0] - w[1]).abs();
            delta >= 1 && delta <= 3
        });

        if correct_diff && (ascending || descending) {
            save_reports += 1;
        }

        line.clear();
    }

    println!("{save_reports}");
}
