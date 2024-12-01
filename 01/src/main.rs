use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let mut line = String::new();

    // read and parse input
    let stdin = io::stdin();
    while let Ok(_) = stdin.read_line(&mut line) {
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        let first = parts.get(0);
        let second = parts.get(1);
        if first.is_some() && second.is_some() {
            let first = first.unwrap().parse::<i64>().unwrap();
            let second = second.unwrap().parse::<i64>().unwrap();
            left.push(Reverse(first));
            right.push(Reverse(second));
        }
        line.clear();
    }

    // calculate distances
    let mut total_distance = 0;
    loop {
        let l = left.pop();
        let r = right.pop();

        if l.is_none() || r.is_none() {
            break;
        }
        let l = l.unwrap().0;
        let r = r.unwrap().0;

        total_distance += (l - r).abs();
    }

    // result
    println!("{total_distance}");
}
