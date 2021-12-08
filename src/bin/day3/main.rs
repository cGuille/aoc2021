fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut positions_counts: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        if positions_counts.len() < line.len() {
            positions_counts.resize(line.len(), (0, 0));
        }

        for (position, char) in line.char_indices() {
            let (mut zeroes, mut ones) = positions_counts[position];

            match char {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Unexpected char {}", char),
            }

            positions_counts[position] = (zeroes, ones);
        }
    }

    let mut gamma = Vec::with_capacity(positions_counts.len());
    let mut epsilon = Vec::with_capacity(positions_counts.len());

    for (zeroes, ones) in positions_counts.into_iter() {
        if zeroes > ones {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = usize::from_str_radix(&gamma.into_iter().collect::<String>(), 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon.into_iter().collect::<String>(), 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn part1_example() {
        assert_eq!(198, part1(EXAMPLE));
    }
}
