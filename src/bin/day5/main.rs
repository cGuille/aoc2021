use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    part(input, |segment| segment.is_axial())
}

fn part2(input: &str) -> usize {
    part(input, |_segment| true)
}

fn part<F>(input: &str, segments_filter: F) -> usize
where F: Fn(&Segment) -> bool
{
    let mut points = HashMap::new();

    input
        .lines()
        .map(|line| {
            line.parse::<Segment>()
                .expect(&format!("Invalid segment: {:?}", line))
        })
        .filter(segments_filter)
        .for_each(|segment| {
            segment.iter().for_each(|p| {
                let count = points.entry(p).or_insert(0);
                *count += 1;
            })
        });

    points.iter().filter(|(_point, count)| count >= &&2).count()
}

#[derive(Debug, PartialEq)]
struct Segment(Point, Point);

impl Segment {
    fn iter(&self) -> SegmentIter {
        SegmentIter {
            started: false,
            current: self.0,
            x_delta: Delta::from(self.0.x, self.1.x),
            y_delta: Delta::from(self.0.y, self.1.y),
        }
    }

    fn is_axial(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
}

#[derive(Debug)]
enum Delta {
    Positive(u32),
    Negative(u32),
    None,
}

impl Delta {
    fn from(start: u32, end: u32) -> Self {
        if start == end {
            Delta::None
        } else if start < end {
            Delta::Positive(end - start)
        } else {
            Delta::Negative(start - end)
        }
    }

    fn decrease(&mut self) {
        match self {
            Self::Positive(ref mut n) if *n > 0 => *n -= 1,
            Self::Negative(ref mut n) if *n > 0 => *n -= 1,
            _ => (),
        };
    }

    fn is_exhausted(&self) -> bool {
        match self {
            Self::None => true,
            Self::Positive(0) => true,
            Self::Negative(0) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct SegmentIter {
    started: bool,
    current: Point,
    x_delta: Delta,
    y_delta: Delta,
}

impl Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;

            return Some(self.current);
        }

        if self.x_delta.is_exhausted() && self.y_delta.is_exhausted() {
            return None;
        }

        self.current = Point {
            x: match self.x_delta {
                Delta::Positive(_) => {
                    self.x_delta.decrease();
                    self.current.x + 1
                }
                Delta::Negative(_) => {
                    self.x_delta.decrease();
                    self.current.x - 1
                }
                Delta::None => self.current.x,
            },
            y: match self.y_delta {
                Delta::Positive(_) => {
                    self.y_delta.decrease();
                    self.current.y + 1
                }
                Delta::Negative(_) => {
                    self.y_delta.decrease();
                    self.current.y - 1
                }
                Delta::None => self.current.y,
            },
        };

        Some(self.current)
    }
}

#[derive(Debug, PartialEq)]
enum ParseSegmentError {
    InvalidFormat,
    InvalidPoint(ParsePointError),
}

impl From<ParsePointError> for ParseSegmentError {
    fn from(err: ParsePointError) -> Self {
        ParseSegmentError::InvalidPoint(err)
    }
}

impl FromStr for Segment {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let point_a: Point = parts
            .next()
            .ok_or(ParseSegmentError::InvalidFormat)?
            .parse()?;

        let point_b: Point = parts
            .next()
            .ok_or(ParseSegmentError::InvalidFormat)?
            .parse()?;

        if parts.next().is_some() {
            Err(ParseSegmentError::InvalidFormat)
        } else {
            Ok(Segment(point_a, point_b))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
enum ParsePointError {
    InvalidFormat,
    InvalidNumber(ParseIntError),
}

impl From<ParseIntError> for ParsePointError {
    fn from(err: ParseIntError) -> Self {
        ParsePointError::InvalidNumber(err)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts
            .next()
            .ok_or(ParsePointError::InvalidFormat)?
            .parse()?;

        let y = parts
            .next()
            .ok_or(ParsePointError::InvalidFormat)?
            .parse()?;

        if parts.next().is_some() {
            Err(ParsePointError::InvalidFormat)
        } else {
            Ok(Point { x, y })
        }
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 12);
    }

    #[test]
    fn parse_point() {
        assert_eq!("3,4".parse(), Ok(Point { x: 3, y: 4 }));

        assert!(matches!(
            "1,2,3".parse::<Point>(),
            Err(ParsePointError::InvalidFormat),
        ));
        assert!(matches!(
            "one,2".parse::<Point>(),
            Err(ParsePointError::InvalidNumber(_)),
        ));
    }

    #[test]
    fn parse_segment() {
        assert_eq!(
            "0,9 -> 5,9".parse(),
            Ok(Segment(Point { x: 0, y: 9 }, Point { x: 5, y: 9 })),
        );

        assert!(matches!(
            "0,9 -> 5,8".parse::<Segment>(),
            Ok(Segment(Point { x: 0, y: 9 }, Point { x: 5, y: 8 })),
        ));

        assert!(matches!(
            "0,9 -> 5,9 -> 0,0".parse::<Segment>(),
            Err(ParseSegmentError::InvalidFormat),
        ));

        assert!(matches!(
            "0,9 -> 5;9".parse::<Segment>(),
            Err(ParseSegmentError::InvalidPoint(_)),
        ));
    }

    #[test]
    fn segment_iter_x() {
        let segment = Segment(Point { x: 0, y: 9 }, Point { x: 2, y: 9 });
        let mut it = segment.iter();

        assert_eq!(it.next(), Some(Point { x: 0, y: 9 }));
        assert_eq!(it.next(), Some(Point { x: 1, y: 9 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 9 }));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        assert_eq!(segment.iter().count(), 3);
    }

    #[test]
    fn segment_iter_y() {
        let segment = Segment(Point { x: 2, y: 9 }, Point { x: 2, y: 5 });
        let mut it = segment.iter();

        assert_eq!(it.next(), Some(Point { x: 2, y: 9 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 8 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 7 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 6 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 5 }));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        assert_eq!(segment.iter().count(), 5);
    }

    #[test]
    fn test_delta() {
        assert!(matches!(Delta::from(0, 3), Delta::Positive(3)));
        assert!(matches!(Delta::from(5, 9), Delta::Positive(4)));
        assert!(matches!(Delta::from(9, 5), Delta::Negative(4)));
        assert!(matches!(Delta::from(5, 5), Delta::None));

        assert!(Delta::None.is_exhausted());
        assert!(Delta::Positive(0).is_exhausted());
        assert!(Delta::Negative(0).is_exhausted());

        assert!(!Delta::Positive(1).is_exhausted());
        assert!(!Delta::Negative(1).is_exhausted());

        let mut delta = Delta::Positive(2);
        assert!(!delta.is_exhausted());

        delta.decrease();
        assert!(matches!(delta, Delta::Positive(1)));
        assert!(!delta.is_exhausted());

        delta.decrease();
        assert!(matches!(delta, Delta::Positive(0)));
        assert!(delta.is_exhausted());
    }
}
