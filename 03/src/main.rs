use std::io;

fn main() {
    let mut line = String::new();
    let mut result1: u64 = 0;

    let r = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    // read and parse input
    let stdin = io::stdin();
    while let Ok(_) = stdin.read_line(&mut line) {
        if line.trim().is_empty() {
            break;
        }

        for (_, [x, y]) in r.captures_iter(&line).map(|c| c.extract()) {
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            result1 += x * y;
        }

        line.clear();
    }

    println!("{result1}");
}
