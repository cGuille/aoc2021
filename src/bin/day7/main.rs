use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> i64 {
    let positions: Vec<i64> = input
        .trim()
        .split(',')
        .map(|part| part.parse().unwrap())
        .collect();

    let lower_bound = *positions.iter().min().unwrap();
    let upper_bound = *positions.iter().max().unwrap();

    let mut selected = upper_bound - lower_bound;
    let mut delta = -(upper_bound - lower_bound) / 2;
    let mut previous_cost = None;

    let mut seen_in_delta_1 = HashSet::new();

    loop {
        let cost: i64 = positions.iter().map(|pos| (selected - pos).abs()).sum();

        if let Some(previous_cost) = previous_cost {
            if cost > previous_cost {
                delta *= -1; // reverse direction

                if delta.abs() > 1 {
                    delta /= 2;
                }
            }
        }

        previous_cost = Some(cost);
        selected += delta;

        if delta.abs() == 1 {
            let already_seen = !seen_in_delta_1.insert(selected);
            if already_seen {
                break;
            }
        }
    }

    positions.iter().map(|pos| (pos - selected).abs()).sum()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 37);
    }
}
