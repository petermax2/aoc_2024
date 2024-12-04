use std::io;

type Puzzle = Vec<Vec<u8>>;

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

fn read_input_from_stdin() -> Puzzle {
    io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.into_bytes())
        .take_while(|line| !line.is_empty())
        .collect()
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
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::NorthEast => (-1, 1),
            Self::East => (0, 1),
            Self::SouthEast => (1, 1),
            Self::South => (1, 0),
            Self::SouthWest => (1, -1),
            Self::West => (0, -1),
            Self::NorthWest => (-1, -1),
        }
    }

    pub fn next_pos(&self, line: usize, pos: usize) -> Option<(usize, usize)> {
        let (dx, dy) = self.offset();
        let new_line = line as isize + dx;
        let new_pos = pos as isize + dy;
        if new_line >= 0 && new_pos >= 0 {
            Some((new_line as usize, new_pos as usize))
        } else {
            None
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
                    if c == X {
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
        let mut xed_mas_count = 0;

        for line in 1..self.lines {
            for pos in 1..self.line_len {
                if let Some(A) = self.get(line, pos) {
                    let diagonals = [
                        SearchDirection::NorthEast,
                        SearchDirection::NorthWest,
                        SearchDirection::SouthEast,
                        SearchDirection::SouthWest,
                    ];

                    let chars: Vec<_> = diagonals
                        .iter()
                        .filter_map(|dir| dir.next_pos(line, pos))
                        .filter_map(|(line, pos)| self.get(line, pos))
                        .collect();

                    if chars.len() == 4 {
                        if (chars[0] == S && chars[3] == M || chars[0] == M && chars[3] == S)
                            && (chars[1] == S && chars[2] == M || chars[1] == M && chars[2] == S)
                        {
                            xed_mas_count += 1;
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
        direction
            .next_pos(line, pos)
            .and_then(|(line, pos)| {
                self.get(line, pos).filter(|c| *c == M).and_then(|_| {
                    direction.next_pos(line, pos).and_then(|(line, pos)| {
                        self.get(line, pos).filter(|c| *c == A).and_then(|_| {
                            direction
                                .next_pos(line, pos)
                                .and_then(|(line, pos)| self.get(line, pos).filter(|c| *c == S))
                        })
                    })
                })
            })
            .map_or(0, |_| 1)
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
