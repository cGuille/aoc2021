use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> u64 {
    input
        .parse::<HeightMap>()
        .unwrap()
        .low_points()
        .iter()
        .map(|measure| measure + 1)
        .sum()
}

#[derive(Debug)]
struct HeightMap {
    column_count: usize,
    cells: Vec<u64>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<u64> {
        let map_width = self.column_count;

        self.cells
            .iter()
            .enumerate()
            .filter(|(index, measure)| {
                [
                    index.checked_sub(map_width),
                    Some(index + 1),
                    Some(index + map_width),
                    index.checked_sub(1),
                ]
                .iter()
                .filter_map(|index| *index)
                .filter_map(|index| self.cells.get(index))
                .all(|adjacent_height| adjacent_height > measure)
            })
            .map(|(_, measure)| *measure)
            .collect()
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
}
