#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input_commands: &str) -> i32 {
    let mut sub = Submarine::default();

    sub.follow_course(input_commands);

    sub.hpos * sub.depth
}

#[derive(Debug, Default)]
struct Submarine {
    hpos: i32,
    depth: i32,
}

impl Submarine {
    fn follow_course(&mut self, course: &str) {
        course
            .lines()
            .map(|line| line.parse().expect("Invalid command"))
            .for_each(|command| self.command(&command))
        ;
    }

    fn command(&mut self, command: &SubmarineCommand) {
        match command {
            SubmarineCommand::Forward(n) => self.hpos += n,
            SubmarineCommand::Up(n) => self.depth -= n,
            SubmarineCommand::Down(n) => self.depth += n,
        };
    }
}

#[derive(Debug, PartialEq)]
enum SubmarineCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, PartialEq)]
enum SubmarineCommandParseError {
    NoMatch,
    UnknownCommand,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for SubmarineCommandParseError {
    fn from(err: ParseIntError) -> Self {
        SubmarineCommandParseError::ParseIntError(err)
    }
}

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"(?P<type>\S+) +(?P<amount>-?\d+)").unwrap();
}

impl FromStr for SubmarineCommand {
    type Err = SubmarineCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = COMMAND_REGEX
            .captures(s)
            .ok_or(SubmarineCommandParseError::NoMatch)?;

        let amount = &caps["amount"].parse()?;

        match &caps["type"] {
            "forward" => Ok(SubmarineCommand::Forward(*amount)),
            "up" => Ok(SubmarineCommand::Up(*amount)),
            "down" => Ok(SubmarineCommand::Down(*amount)),
            _ => Err(SubmarineCommandParseError::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn parse_command() {
        assert_eq!(Ok(SubmarineCommand::Up(10)), "up 10".parse(), "Up with positive amount");
        assert_eq!(Ok(SubmarineCommand::Up(-10)), "up -10".parse(), "Up with negative amount");
        assert_eq!(Ok(SubmarineCommand::Down(5)), "down 5".parse(), "Down with positive amount");
        assert_eq!(Ok(SubmarineCommand::Forward(5)), "forward 5".parse(), "Forward with positive amount");
        assert_eq!(Err(SubmarineCommandParseError::NoMatch), "not a command".parse::<SubmarineCommand>(), "String not matching");
        assert_eq!(Err(SubmarineCommandParseError::UnknownCommand), "not_a_command 10".parse::<SubmarineCommand>(), "Invalid command");
    }

    #[test]
    fn sub_commanding() {
        let mut sub = Submarine::default();

        sub.command(&SubmarineCommand::Down(10));
        assert_eq!(0, sub.hpos);
        assert_eq!(10, sub.depth);

        sub.command(&SubmarineCommand::Forward(100));
        assert_eq!(100, sub.hpos);
        assert_eq!(10, sub.depth);

        sub.command(&SubmarineCommand::Up(3));
        assert_eq!(100, sub.hpos);
        assert_eq!(7, sub.depth);

        sub.command(&SubmarineCommand::Forward(-20));
        assert_eq!(80, sub.hpos);
        assert_eq!(7, sub.depth);

        sub.command(&SubmarineCommand::Down(200));
        assert_eq!(80, sub.hpos);
        assert_eq!(207, sub.depth);
    }

    #[test]
    fn part1_example() {
        assert_eq!(150, part1(EXAMPLE));
    }
}
