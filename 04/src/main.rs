use std::io;

type Puzzle = Vec<Vec<u8>>;

fn read_input_from_stdin() -> Puzzle {
    let mut line = String::new();
    let mut input = Vec::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut line).is_ok() {
        if line.trim().is_empty() {
            break;
        }
        input.push(line.clone().into_bytes());
        line.clear();
    }
    input
}

#[derive(Debug)]
enum SearchDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl SearchDirection {
    pub fn next_pos(&self, line: usize, pos: usize) -> Option<(usize, usize)> {
        match &self {
            SearchDirection::North => {
                if line > 0 {
                    Some((line - 1, pos))
                } else {
                    None
                }
            }
            SearchDirection::NorthEast => {
                if line > 0 {
                    Some((line - 1, pos + 1))
                } else {
                    None
                }
            }
            SearchDirection::East => Some((line, pos + 1)),
            SearchDirection::SouthEast => Some((line + 1, pos + 1)),
            SearchDirection::South => Some((line + 1, pos)),
            SearchDirection::SouthWest => {
                if pos > 0 {
                    Some((line + 1, pos - 1))
                } else {
                    None
                }
            }
            SearchDirection::West => {
                if pos > 0 {
                    Some((line, pos - 1))
                } else {
                    None
                }
            }
            SearchDirection::NorthWest => {
                if pos > 0 && line > 0 {
                    Some((line - 1, pos - 1))
                } else {
                    None
                }
            }
        }
    }
}

struct PuzzleNavigator<'a> {
    puzzle: &'a Puzzle,
    lines: usize,
    line_len: usize,
}

impl<'a> PuzzleNavigator<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        let lines = puzzle.len();
        if lines == 0 {
            panic!("Input must not be empty");
        }

        let line_len = puzzle[0].len();
        if line_len == 0 {
            panic!("Input line must not be zero");
        }

        Self {
            puzzle,
            lines,
            line_len,
        }
    }

    pub fn xmas_count(&self) -> u64 {
        let mut xmas_count: u64 = 0;
        for line in 0..self.lines {
            for pos in 0..self.line_len {
                if let Some(c) = self.get(line, pos) {
                    if c == b'X' {
                        xmas_count += self.detect_mas(SearchDirection::East, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::North, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::NorthEast, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::NorthWest, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::South, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::SouthEast, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::SouthWest, line, pos);
                        xmas_count += self.detect_mas(SearchDirection::West, line, pos);
                    }
                }
            }
        }
        xmas_count
    }

    pub fn mas_count_crossed(&self) -> u64 {
        let mut xed_mas_count: u64 = 0;
        for line in 1..self.lines {
            for pos in 1..self.line_len {
                if let Some(c) = self.get(line, pos) {
                    if c == b'A' {
                        let ne = SearchDirection::NorthEast.next_pos(line, pos);
                        let nw = SearchDirection::NorthWest.next_pos(line, pos);
                        let se = SearchDirection::SouthEast.next_pos(line, pos);
                        let sw = SearchDirection::SouthWest.next_pos(line, pos);

                        if ne.is_some() && nw.is_some() && se.is_some() && sw.is_some() {
                            let ne = ne.unwrap();
                            let nw = nw.unwrap();
                            let se = se.unwrap();
                            let sw = sw.unwrap();

                            let ne = self.get(ne.0, ne.1);
                            let nw = self.get(nw.0, nw.1);
                            let se = self.get(se.0, se.1);
                            let sw = self.get(sw.0, sw.1);

                            if ne.is_some() && nw.is_some() && se.is_some() && sw.is_some() {
                                let ne = ne.unwrap();
                                let nw = nw.unwrap();
                                let se = se.unwrap();
                                let sw = sw.unwrap();

                                let diagonal1_mas =
                                    (ne == b'S' && sw == b'M') || (ne == b'M' && sw == b'S');
                                let diagonal2_mas =
                                    (nw == b'S' && se == b'M') || (nw == b'M' && se == b'S');

                                if diagonal1_mas && diagonal2_mas {
                                    xed_mas_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        xed_mas_count
    }

    fn get(&self, line: usize, pos: usize) -> Option<u8> {
        if line < self.lines && pos < self.line_len {
            Some(self.puzzle[line][pos])
        } else {
            None
        }
    }

    fn detect_mas(&self, direction: SearchDirection, line: usize, pos: usize) -> u64 {
        if let Some((line, pos)) = direction.next_pos(line, pos) {
            if let Some(x) = self.get(line, pos) {
                if x != b'M' {
                    return 0;
                }
            } else {
                return 0;
            }

            if let Some((line, pos)) = direction.next_pos(line, pos) {
                if let Some(x) = self.get(line, pos) {
                    if x != b'A' {
                        return 0;
                    }
                } else {
                    return 0;
                }

                if let Some((line, pos)) = direction.next_pos(line, pos) {
                    if let Some(x) = self.get(line, pos) {
                        if x != b'S' {
                            return 0;
                        }
                    } else {
                        return 0;
                    }
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }

        1
    }
}

fn main() {
    let input = read_input_from_stdin();

    let navigator = PuzzleNavigator::new(&input);
    let xmas_count = navigator.xmas_count();
    let crossed_mas_count = navigator.mas_count_crossed();

    println!("{xmas_count}");
    println!("{crossed_mas_count}");
}
