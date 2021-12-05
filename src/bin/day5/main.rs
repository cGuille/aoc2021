use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut points = HashMap::new();

    input
        .lines()
        .map(|line| {
            line.parse::<Segment>()
                .expect(&format!("Invalid segment: {:?}", line))
        })
        .filter(|segment| segment.is_axial())
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
        let (direction, val0, val1) = if self.0.x == self.1.x {
            (SegmentIterDirection::Vertical, self.0.y, self.1.y)
        } else {
            (SegmentIterDirection::Horizontal, self.0.x, self.1.x)
        };

        let (start, end) = if val0 < val1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        };

        SegmentIter {
            current: start,
            direction,
            end,
            end_reached: false,
        }
    }

    fn is_axial(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
}

#[derive(Debug)]
enum SegmentIterDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct SegmentIter {
    current: Point,
    direction: SegmentIterDirection,
    end: Point,
    end_reached: bool,
}

impl Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached {
            return None;
        }

        let item = self.current;

        match self.direction {
            SegmentIterDirection::Horizontal => {
                self.current = Point {
                    x: self.current.x + 1,
                    y: self.current.y,
                };

                if self.current.x > self.end.x {
                    self.end_reached = true;
                }
            }
            SegmentIterDirection::Vertical => {
                self.current = Point {
                    x: self.current.x,
                    y: self.current.y + 1,
                };

                if self.current.y > self.end.y {
                    self.end_reached = true;
                }
            }
        }

        Some(item)
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

        assert_eq!(it.next(), Some(Point { x: 2, y: 5 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 6 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 7 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 8 }));
        assert_eq!(it.next(), Some(Point { x: 2, y: 9 }));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        assert_eq!(segment.iter().count(), 5);
    }
}
