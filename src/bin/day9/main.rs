use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u64 {
    input
        .parse::<HeightMap>()
        .unwrap()
        .low_points()
        .iter()
        .map(|(_, measure)| measure + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let basins = input.parse::<HeightMap>().unwrap().basins();

    let mut basin_sizes: Vec<_> = basins.into_iter().map(|basin| basin.len()).collect();

    basin_sizes.sort_unstable();

    basin_sizes.iter().rev().take(3).product()
}

#[derive(Debug)]
struct HeightMap {
    column_count: usize,
    cells: Vec<u64>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<(usize, u64)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(index, measure)| {
                [
                    index.checked_sub(self.column_count),
                    if ((index + 1) % self.column_count) == 0 {
                        None
                    } else {
                        Some(index + 1)
                    },
                    Some(index + self.column_count),
                    if (index % self.column_count) == 0 {
                        None
                    } else {
                        index.checked_sub(1)
                    },
                ]
                .iter()
                .filter_map(|index| *index)
                .filter_map(|index| self.cells.get(index))
                .all(|adjacent_height| adjacent_height > measure)
            })
            .map(|(index, measure)| (index, *measure))
            .collect()
    }

    fn basins(&self) -> Vec<Vec<(usize, u64)>> {
        self.low_points()
            .iter()
            .map(|(index, _)| {
                let mut visited = HashSet::new();
                self.scan_basin(*index, &mut visited);
                visited.into_iter().map(|i| (i, self.cells[i])).collect()
            })
            .collect()
    }

    fn scan_basin(&self, index: usize, visited: &mut HashSet<usize>) {
        visited.insert(index);

        let newly_visited: Vec<_> = [
            index.checked_sub(self.column_count),
            if ((index + 1) % self.column_count) == 0 {
                None
            } else {
                Some(index + 1)
            },
            Some(index + self.column_count),
            if (index % self.column_count) == 0 {
                None
            } else {
                index.checked_sub(1)
            },
        ]
        .iter()
        .filter_map(|index| *index)
        .filter(|adj_index| match self.cells.get(*adj_index) {
            Some(measure) => *measure < 9,
            None => false,
        })
        .filter(|adj_index| visited.insert(*adj_index))
        .collect();

        newly_visited
            .iter()
            .for_each(|new_index| self.scan_basin(*new_index, visited));
    }
}

#[derive(Debug)]
enum ParseHeightMapError {
    InvalidInput,
    InvalidMeasure,
}

impl FromStr for HeightMap {
    type Err = ParseHeightMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseHeightMapError::*;

        let mut lines = s.lines().peekable();

        let first_line = lines.peek().ok_or(InvalidInput)?;
        let column_count = first_line.len();

        let cells: Vec<u64> = lines
            .map(|line| line.chars())
            .flatten()
            .map(|c| c.to_digit(10).ok_or(InvalidMeasure).map(u64::from))
            .collect::<Result<_, ParseHeightMapError>>()?;

        Ok(Self {
            cells,
            column_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_parse_height_map() {
        let result: Result<HeightMap, _> = EXAMPLE.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1134);
    }
}
