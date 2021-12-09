use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut fishes: Vec<Lanternfish> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..80 {
        let newborns: Vec<_> = fishes.iter_mut().flat_map(|fish| fish.next_day()).collect();
        fishes.extend(newborns);
    }

    fishes.len()
}

struct Lanternfish {
    term: u8,
}

impl Lanternfish {
    fn newborn() -> Self {
        Lanternfish { term: 8 }
    }

    fn next_day(&mut self) -> Option<Lanternfish> {
        if self.term == 0 {
            self.term = 6;

            Some(Self::newborn())
        } else {
            self.term -= 1;

            None
        }
    }
}

impl FromStr for Lanternfish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lanternfish { term: s.parse()? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3,4,3,1,2
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 5934);
    }

    #[test]
    fn lanterfish_test() {
        let mut fish = Lanternfish::newborn();

        for _ in 0..8 {
            assert!(fish.next_day().is_none());
        }

        assert!(matches!(fish.next_day(), Some(Lanternfish { term: 8 })));
        assert!(matches!(fish, Lanternfish { term: 6 }));
    }
}
