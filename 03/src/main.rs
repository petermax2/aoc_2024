use std::io;

fn main() {
    let mut line = String::new();
    let mut filtered_input = String::new();
    let mut input = String::new();
    let mut result1: u64 = 0;
    let mut result2: u64 = 0;

    let r = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    // read and parse input
    let stdin = io::stdin();
    while stdin.read_line(&mut line).is_ok() {
        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line);
        line.clear();
    }

    // part 1
    for (_, [x, y]) in r.captures_iter(&input).map(|c| c.extract()) {
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();
        result1 += x * y;
    }

    // part 2
    let mut line_rest = input.as_str();
    loop {
        if let Some(index) = line_rest.find("don't()") {
            filtered_input.push_str(&line_rest[0..index]);
            line_rest = &line_rest[index..];
            if let Some(index) = line_rest.find("do()") {
                line_rest = &line_rest[index..];
            } else {
                break;
            }
        } else {
            filtered_input.push_str(&line_rest);
            break;
        }
    }
    for (_, [x, y]) in r.captures_iter(&filtered_input).map(|c| c.extract()) {
        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();
        result2 += x * y;
    }

    // result
    println!("{result1}");
    println!("{result2}");
}
