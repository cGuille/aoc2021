fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut stack = Vec::new();
    let mut illegal_score = 0;

    for line in input.lines() {
        stack.clear();

        for symbol in line.chars().map(|c| Symbol::try_from(c).unwrap()) {
            if symbol.is_opening() {
                stack.push(symbol);
                continue;
            }

            let opening = stack.pop();
            if opening.is_none() {
                // incomplete?
                continue;
            }

            let opening = opening.unwrap();

            if !opening.is_closed_by(&symbol) {
                illegal_score += symbol.illegal_score();
            }
        }
    }

    illegal_score
}

#[derive(Debug)]
struct ParseSymbolError;

#[derive(Debug)]
enum Symbol {
    Opening(char),
    Closing(char),
}

impl Symbol {
    fn is_opening(&self) -> bool {
        match self {
            Self::Opening(_) => true,
            Self::Closing(_) => false,
        }
    }

    fn is_closed_by(&self, other: &Symbol) -> bool {
        match self {
            Self::Closing(_) => false,
            Self::Opening(opening) => match other {
                Self::Opening(_) => false,
                Self::Closing(closing) => match (opening, closing) {
                    ('<', '>') => true,
                    ('(', ')') => true,
                    ('[', ']') => true,
                    ('{', '}') => true,
                    _ => false,
                },
            },
        }
    }

    fn illegal_score(&self) -> u64 {
        match self {
            Self::Opening(_) => 0,
            Self::Closing(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1_197,
                '>' => 25_137,
                _ => 0,
            },
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = ParseSymbolError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' | '(' | '[' | '{' => Ok(Self::Opening(c)),
            '>' | ')' | ']' | '}' => Ok(Self::Closing(c)),
            _ => Err(ParseSymbolError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 26397);
    }
}
