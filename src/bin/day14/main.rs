use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

type Rules = HashMap<(char, char), char>;

fn parse(input: &str) -> (String, Rules) {
    let mut lines = input.lines();

    let template = lines.next().unwrap().into();

    assert!(matches!(lines.next(), Some("")));

    let rules = lines
        .map(|line| {
            let mut parts = line.split(" -> ");

            let pattern = parts.next().unwrap();
            let insertion = parts.next().unwrap();

            assert!(parts.next().is_none());

            let mut chars = pattern.chars();
            let c1 = chars.next().unwrap();
            let c2 = chars.next().unwrap();

            assert!(chars.next().is_none());

            let mut chars = insertion.chars();
            let insertion = chars.next().unwrap();

            assert!(chars.next().is_none());

            ((c1, c2), insertion)
        })
        .collect();

    (template, rules)
}

fn part1(input: &str) -> u64 {
    let (mut template, rules) = parse(input);

    for _step in 1..=10 {
        template = apply(&rules, template);
    }

    let mut occurrences: HashMap<char, u64> = HashMap::new();

    for c in template.chars() {
        let c_occurrences = occurrences.entry(c).or_default();

        *c_occurrences += 1;
    }

    let (min, max) = occurrences
        .iter()
        .fold((u64::MAX, u64::MIN), |(min, max), (_c, current)| {
            (
                if *current < min { *current } else { min },
                if *current > max { *current } else { max },
            )
        });

    max - min
}

fn apply(rules: &Rules, template: String) -> String {
    template
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .fold(
            template.chars().next().unwrap().to_string(),
            |acc, pattern| {
                let mut pattern_chars = pattern.iter();

                let c1 = pattern_chars.next().unwrap();
                let c2 = pattern_chars.next().unwrap();

                match rules.get(&(*c1, *c2)) {
                    Some(insertion) => format!("{}{}{}", acc, insertion, c2),
                    None => format!("{}{}", acc, c2),
                }
            },
        )
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
