use std::collections::HashMap;
use std::io;

#[derive(PartialEq, Eq, Debug)]
struct PageOrderingRules {
    rules: HashMap<u64, Vec<u64>>,
    empty_vec: Vec<u64>,
}

impl PageOrderingRules {
    fn get_succesors(&self, number: u64) -> &Vec<u64> {
        self.rules.get(&number).unwrap_or(&self.empty_vec)
    }
}

impl From<Vec<String>> for PageOrderingRules {
    fn from(values: Vec<String>) -> Self {
        let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();

        for line in values {
            let parts: Vec<u64> = line
                .split("|")
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if let [a, b] = parts[..] {
                if let Some(list) = rules.get_mut(&a) {
                    list.push(b);
                } else {
                    rules.insert(a, vec![b]);
                }
            }
        }

        Self {
            rules,
            empty_vec: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct ManualUpdate {
    pub pages: Vec<u64>,
}

impl ManualUpdate {
    pub fn middle_page(&self) -> u64 {
        *self.pages.get((self.pages.len() - 1) / 2).unwrap_or(&0)
    }
}

impl From<&str> for ManualUpdate {
    fn from(value: &str) -> Self {
        let pages: Vec<u64> = value
            .split(",")
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        Self { pages }
    }
}

#[derive(Debug)]
struct Puzzle {
    rules: PageOrderingRules,
    pages: Vec<ManualUpdate>,
}

impl Puzzle {
    pub fn new(rules: PageOrderingRules, pages: Vec<ManualUpdate>) -> Self {
        Self { rules, pages }
    }

    pub fn correctly_ordered_middle_number_sum(&self) -> u64 {
        self.pages
            .iter()
            .filter(|p| {
                for i in 0..p.pages.len() {
                    let number = *p.pages.get(i).unwrap();
                    let succesors = self.rules.get_succesors(number);
                    if p.pages[0..i].iter().any(|n| succesors.contains(n)) {
                        return false;
                    }
                }
                true
            })
            .map(|p| p.middle_page())
            .sum()
    }
}

fn read_puzzle_from_stdin() -> Puzzle {
    let lines = io::stdin().lines().filter_map(|line| line.ok());

    let mut rules: Vec<String> = Vec::new();
    let mut pages = Vec::new();

    let mut update_mode = false;
    for line in lines {
        if update_mode {
            if line.trim().is_empty() {
                break;
            }
            pages.push(ManualUpdate::from(line.as_str()));
        } else {
            if line.trim().is_empty() {
                update_mode = true;
                continue;
            }
            rules.push(line);
        }
    }

    Puzzle::new(PageOrderingRules::from(rules), pages)
}

fn main() {
    let puzzle = read_puzzle_from_stdin();
    let correct_middle_numbers_sum = puzzle.correctly_ordered_middle_number_sum();

    println!("{correct_middle_numbers_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pagerules() {
        let actual = PageOrderingRules::from(vec![
            "47|53".to_owned(),
            "97|13".to_owned(),
            "97|61".to_owned(),
        ]);

        let mut rules = HashMap::new();
        rules.insert(47, vec![53]);
        rules.insert(97, vec![13, 61]);
        let expected = PageOrderingRules {
            rules: rules,
            empty_vec: Vec::new(),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_update() {
        let actual = ManualUpdate::from("75,47,61,53,29");

        let expected = ManualUpdate {
            pages: vec![75, 47, 61, 53, 29],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_middle_page() {
        let test = ManualUpdate {
            pages: vec![75, 47, 61, 53, 29],
        };
        assert_eq!(61, test.middle_page());

        let test = ManualUpdate {
            pages: vec![75, 29, 13],
        };
        assert_eq!(29, test.middle_page());

        let test = ManualUpdate {
            pages: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(5, test.middle_page());
    }
}
