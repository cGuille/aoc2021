fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1588);
    }
}
