fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(measures_input: &str) -> u32 {
    measures_input
        .lines()
        .map(|line| line.parse::<i32>().expect("Could not parse line into i32"))
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .fold(0, |increase_count, window| match window {
            [a, b] if a < b => increase_count + 1,
            _ => increase_count,
        })
}

fn part2(measures_input: &str) -> u32 {
    measures_input
        .lines()
        .map(|line| line.parse::<i32>().expect("Could not parse line into i32"))
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .fold(0, |increase_count, window| match window {
            [a, b] if a < b => increase_count + 1,
            _ => increase_count,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_MEASURES: &str = "\
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn part1_example() {
        assert_eq!(7, part1(EXAMPLE_MEASURES));
    }

    #[test]
    fn part2_example() {
        assert_eq!(5, part2(EXAMPLE_MEASURES));
    }
}
