#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input_commands: &str) -> i32 {
    part(input_commands, Box::new(SubmarinePart1::default()))
}

fn part2(input_commands: &str) -> i32 {
    part(input_commands, Box::new(SubmarinePart2::default()))
}

fn part(input_commands: &str, mut sub: Box<dyn Submarine>) -> i32 {
    sub.follow_course(input_commands);

    sub.hpos() * sub.depth()
}

trait Submarine {
    fn follow_course(&mut self, course: &str) {
        course
            .lines()
            .map(|line| line.parse().expect("Invalid command"))
            .for_each(|command| self.command(&command))
        ;
    }

    fn command(&mut self, command: &SubmarineCommand);
    fn hpos(&self) -> i32;
    fn depth(&self) -> i32;
}

#[derive(Debug, Default)]
struct SubmarinePart1 {
    hpos: i32,
    depth: i32,
}

impl Submarine for SubmarinePart1 {
    fn command(&mut self, command: &SubmarineCommand) {
        match command {
            SubmarineCommand::Up(n) => self.depth -= n,
            SubmarineCommand::Down(n) => self.depth += n,
            SubmarineCommand::Forward(n) => self.hpos += n,
        };
    }

    fn hpos(&self) -> i32 {
        self.hpos
    }

    fn depth(&self) -> i32 {
        self.depth
    }
}

#[derive(Debug, Default)]
struct SubmarinePart2 {
    aim: i32,
    hpos: i32,
    depth: i32,
}

impl Submarine for SubmarinePart2 {
    fn command(&mut self, command: &SubmarineCommand) {
        match command {
            SubmarineCommand::Up(n) => self.aim -= n,
            SubmarineCommand::Down(n) => self.aim += n,
            SubmarineCommand::Forward(n) => {
                self.hpos += n;
                self.depth += self.aim * n;
            },
        };
    }

    fn hpos(&self) -> i32 {
        self.hpos
    }

    fn depth(&self) -> i32 {
        self.depth
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
mod tests {
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
    fn sub_commanding_part1() {
        let mut sub = SubmarinePart1::default();

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

    #[test]
    fn part2_example() {
        assert_eq!(900, part2(EXAMPLE));
    }
}
