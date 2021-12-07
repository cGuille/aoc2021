use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i64 {
    part(input, |distance| distance)
}

fn part2(input: &str) -> i64 {
    part(input, n_first_int_sum)
}

fn part<F>(input: &str, cost: F) -> i64
where
    F: Fn(i64) -> i64,
{
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
        let total_cost: i64 = positions
            .iter()
            .map(|pos| cost((selected - pos).abs()))
            .sum();

        if let Some(previous_cost) = previous_cost {
            if total_cost > previous_cost {
                delta *= -1; // reverse direction

                if delta.abs() > 1 {
                    delta /= 2;
                }
            }
        }

        previous_cost = Some(total_cost);
        selected += delta;

        if delta.abs() == 1 {
            let already_seen = !seen_in_delta_1.insert(selected);
            if already_seen {
                break;
            }
        }
    }

    positions
        .iter()
        .map(|pos| cost((pos - selected).abs()))
        .sum()
}

/// Cf. https://fr.wikipedia.org/wiki/Somme_(arithm%C3%A9tique)#Somme_des_premiers_entiers
fn n_first_int_sum(n: i64) -> i64 {
    (n * (n + 1)) / 2
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 168);
    }

    #[test]
    fn test_n_first_int_sum() {
        assert_eq!(n_first_int_sum(1), 1);
        assert_eq!(n_first_int_sum(2), 3);
        assert_eq!(n_first_int_sum(3), 6);
        assert_eq!(n_first_int_sum(4), 10);
        assert_eq!(n_first_int_sum(100), 5050);

        assert_eq!(n_first_int_sum((16i64 - 5).abs()), 66);
        assert_eq!(n_first_int_sum((1i64 - 5).abs()), 10);
        assert_eq!(n_first_int_sum((14i64 - 5).abs()), 45);
    }
}
