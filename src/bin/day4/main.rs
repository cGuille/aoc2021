use core::panic;

fn main() {
    let input = include_str!("input.txt");

    let (draw, boards) = parse_input(input);

    println!("{}", part1(&draw, boards.clone()));
    println!("{}", part2(&draw, boards));
}

fn parse_input(input: &str) -> (bingo::Draw, Vec<bingo::Board>) {
    let mut lines = input.lines();

    let draw: bingo::Draw = lines
        .next()
        .expect("Invalid input: no draw")
        .parse()
        .expect("Invalid draw");

    let delimiter = lines.next().expect("No boards after draw");
    assert_eq!(delimiter, "", "Line after draw should be empty");

    let mut boards: Vec<bingo::Board> = Vec::new();

    loop {
        let board_str = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join(" ");

        if board_str.is_empty() {
            break;
        }

        boards.push(board_str.parse().expect("Invalid board"));
    }

    (draw, boards)
}

fn part1(draw: &bingo::Draw, mut boards: Vec<bingo::Board>) -> u64 {
    for n in draw.iter() {
        for board in boards.iter_mut() {
            board.mark(*n);

            if board.won() {
                return board.score() * n;
            }
        }
    }

    panic!("Winning board not found!")
}

fn part2(draw: &bingo::Draw, mut boards: Vec<bingo::Board>) -> u64 {
    for n in draw.iter() {
        let mut to_remove = Vec::new();
        let boards_len = boards.len();

        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(*n);

            if board.won() {
                to_remove.push(i);

                if boards_len - to_remove.len() == 0 {
                    return board.score() * n;
                }
            }
        }

        to_remove.sort();
        to_remove.reverse();

        for board_index in to_remove.iter() {
            boards.remove(*board_index);
        }
    }

    panic!("Latest winning board not found!")
}

mod bingo {
    use std::{
        fmt::{Display, Formatter},
        num::ParseIntError,
        str::FromStr,
    };

    #[derive(Debug)]
    pub struct Draw(Vec<u64>);

    impl Draw {
        pub fn iter(&self) -> std::slice::Iter<u64> {
            self.0.iter()
        }
    }

    impl FromStr for Draw {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Draw(
                s.trim()
                    .split(',')
                    .map(|part| part.parse::<u64>())
                    .collect::<Result<Vec<u64>, ParseIntError>>()?,
            ))
        }
    }

    #[derive(Clone, Debug)]
    enum BoardCell {
        Number(u64),
        Marked,
    }

    impl BoardCell {
        fn is_marked(&self) -> bool {
            if let Self::Marked = self {
                true
            } else {
                false
            }
        }

        fn mark_if(&mut self, number: u64) {
            if let Self::Number(n) = self {
                if *n == number {
                    *self = Self::Marked;
                }
            }
        }
    }

    impl Display for BoardCell {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Self::Number(n) => write!(f, "{}", n)?,
                Self::Marked => write!(f, "x")?,
            };

            Ok(())
        }
    }

    impl FromStr for BoardCell {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(BoardCell::Number(s.parse()?))
        }
    }

    #[derive(Clone, Debug)]
    pub struct Board(Vec<BoardCell>);

    impl Display for Board {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for (i, cell) in self.0.iter().enumerate() {
                let delim = if (i + 1) % 5 == 0 { '\n' } else { ' ' };
                write!(f, "{}{}", cell, delim)?;
            }

            Ok(())
        }
    }

    impl Board {
        pub fn mark(&mut self, number: u64) {
            self.0.iter_mut().for_each(|cell| cell.mark_if(number))
        }

        pub fn won(&self) -> bool {
            self.rows()
                .chain(self.columns())
                .any(|cells| cells.iter().all(|cell| cell.is_marked()))
        }

        pub fn score(&self) -> u64 {
            self.0
                .iter()
                .filter_map(|cell| match cell {
                    BoardCell::Marked => None,
                    BoardCell::Number(n) => Some(n),
                })
                .sum()
        }

        fn rows(&self) -> BoardRows {
            BoardRows::new(self)
        }

        fn columns(&self) -> BoardColumns {
            BoardColumns::new(self)
        }
    }

    impl FromStr for Board {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Board(
                s.split(char::is_whitespace)
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse())
                    .collect::<Result<Vec<BoardCell>, ParseIntError>>()?,
            ))
        }
    }

    struct BoardRows<'a> {
        board: &'a Board,
        it_count: usize,
    }

    impl<'a> BoardRows<'a> {
        fn new(board: &'a Board) -> Self {
            BoardRows { board, it_count: 0 }
        }
    }

    impl<'a> Iterator for BoardRows<'a> {
        type Item = Vec<&'a BoardCell>;

        fn next(&mut self) -> Option<Self::Item> {
            let start = self.it_count * 5;
            let end = start + 5;

            if end > self.board.0.len() {
                return None;
            }

            let cells: Vec<&'a BoardCell> = self.board.0[start..end].iter().collect();

            self.it_count += 1;

            Some(cells)
        }
    }

    struct BoardColumns<'a> {
        board: &'a Board,
        it_count: usize,
    }

    impl<'a> BoardColumns<'a> {
        fn new(board: &'a Board) -> Self {
            BoardColumns { board, it_count: 0 }
        }
    }

    impl<'a> Iterator for BoardColumns<'a> {
        type Item = Vec<&'a BoardCell>;

        fn next(&mut self) -> Option<Self::Item> {
            let i = self.it_count;

            self.it_count += 1;

            (0..5)
                .map(|n| self.board.0.get(n * 5 + i))
                .collect::<Option<Vec<&'a BoardCell>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_draw() {
            let draw: Draw = "1,2,3".parse().expect("Parsing failed");
            assert_eq!(draw.0.len(), 3);
        }

        #[test]
        fn parse_board() {
            let board_lines =
                "22 13 17 11  0 8  2 23  4 24 21  9 14 16  7 6 10  3 18  5 1 12 20 15 19";
            let board: Board = board_lines.parse().expect("Parsing failed");

            assert_eq!(board.0.len(), 5 * 5);
        }
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn part1_example() {
        let (draw, boards) = parse_input(EXAMPLE);

        assert_eq!(4512, part1(&draw, boards));
    }

    #[test]
    fn part2_example() {
        let (draw, boards) = parse_input(EXAMPLE);

        assert_eq!(1924, part2(&draw, boards));
    }
}
