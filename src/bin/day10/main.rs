use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
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

fn part2(input: &str) -> u64 {
    let mut stack;
    let mut line_scores = Vec::new();

    'line_loop: for line in input.lines() {
        stack = Vec::new();

        'symbol_loop: for symbol in line.chars().map(|c| Symbol::try_from(c).unwrap()) {
            if symbol.is_opening() {
                stack.push(symbol);
                continue 'symbol_loop; // go to next symbol in the line
            }

            // here symbol is a closing one

            let opening = stack.pop();

            if opening.is_none() {
                // we don't have any opening symbol for this closing symbol
                continue 'line_loop; // go to next line in input
            }

            let opening = opening.unwrap();
            if !opening.is_closed_by(&symbol) {
                // corrupted: this closing symbol does not match its opening
                continue 'line_loop; // go to next line in input
            }
        }

        if stack.is_empty() {
            // all opening symbols have had a matching closing symbol for this
            // line, so there is nothing to complete
            continue 'line_loop; // go to next line in input
        }

        line_scores.push(
            stack
                .into_iter()
                .rfold(0, |acc, symbol| acc * 5 + symbol.autocomplete_score()),
        );
    }

    line_scores.sort_unstable();

    line_scores[line_scores.len() / 2]
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
                Self::Closing(closing) => matches!(
                    (opening, closing),
                    ('<', '>') | ('(', ')') | ('[', ']') | ('{', '}')
                ),
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

    fn autocomplete_score(&self) -> u64 {
        match self {
            Self::Closing(_) => 0,
            Self::Opening(c) => match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            },
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Opening(c) => c,
                Self::Closing(c) => c,
            }
        )
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 288957);
    }
}
