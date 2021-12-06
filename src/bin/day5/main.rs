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
    fn is_axial(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn iter(&self) -> SegmentIter {
        SegmentIter {
            segment: &self,
            iter_count: 0,
            ended: false,
        }
    }
}

#[derive(Debug)]
struct SegmentIter<'a> {
    segment: &'a Segment,
    iter_count: i64,
    ended: bool,
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        let x = iter_coord(self.iter_count, self.segment.0.x, self.segment.1.x);
        let y = iter_coord(self.iter_count, self.segment.0.y, self.segment.1.y);

        let current = Point { x, y };

        self.iter_count += 1;

        if current == self.segment.1 {
            self.ended = true;
        }

        Some(current)
    }
}

fn iter_coord(iter_count: i64, start: i64, end: i64) -> i64 {
    if start <= end {
        std::cmp::min(start + iter_count, end)
    } else {
        std::cmp::max(start - iter_count, end)
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
    x: i64,
    y: i64,
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
}
