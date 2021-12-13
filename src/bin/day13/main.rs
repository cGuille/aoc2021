use std::{collections::HashSet, fmt::Debug, num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let dots: HashSet<Dot> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let folds: Vec<Fold> = lines.map(|line| line.parse().unwrap()).collect();

    let dots = fold(dots, folds.get(0).unwrap());

    dots.len()
}

fn fold(dots: HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
    dots.into_iter().map(|dot| dot.fold(fold)).collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Dot {
    x: i64,
    y: i64,
}

impl Dot {
    fn fold(&self, fold: &Fold) -> Self {
        match fold {
            Fold::Up(threshold) => Self {
                x: self.x,
                y: if self.y > *threshold {
                    *threshold - (self.y - *threshold)
                } else {
                    self.y
                },
            },
            Fold::Left(threshold) => Self {
                x: if self.x > *threshold {
                    *threshold - (self.x - *threshold)
                } else {
                    self.x
                },
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
enum ParseDotError {
    InvalidFormat,
    InvalidInt(ParseIntError),
}

impl From<ParseIntError> for ParseDotError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInt(e)
    }
}

impl FromStr for Dot {
    type Err = ParseDotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts.next().ok_or(Self::Err::InvalidFormat)?.parse()?;
        let y = parts.next().ok_or(Self::Err::InvalidFormat)?.parse()?;

        if parts.next().is_some() {
            Err(Self::Err::InvalidFormat)
        } else {
            Ok(Self { x, y })
        }
    }
}

#[derive(Debug)]
enum Fold {
    Up(i64),
    Left(i64),
}

#[derive(Debug)]
enum ParseFoldError {
    InvalidFormat,
    InvalidInt(ParseIntError),
}

impl From<ParseIntError> for ParseFoldError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInt(e)
    }
}

impl FromStr for Fold {
    type Err = ParseFoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("fold along ") {
            return Err(Self::Err::InvalidFormat);
        }

        let mut parts = s[11..].split('=');

        let axis = parts.next().ok_or(Self::Err::InvalidFormat)?;
        let threshold = parts.next().ok_or(Self::Err::InvalidFormat)?.parse()?;

        if parts.next().is_some() {
            return Err(Self::Err::InvalidFormat);
        }

        match axis {
            "x" => Ok(Fold::Left(threshold)),
            "y" => Ok(Fold::Up(threshold)),
            _ => Err(Self::Err::InvalidFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 17);
    }
}
