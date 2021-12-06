#![allow(unused_variables)]

use common::read_puzzle_input;

#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.0.1 == self.1.1
    }

    fn is_vertical(&self) -> bool {
        self.0.0 == self.1.0
    }
}

#[derive(Debug, PartialEq)]
struct Point(usize, usize);

fn main() {
    let input_entries = read_puzzle_input(5);
    let first_lines: Vec<Line> = parse_vent_lines(input_entries).into_iter().filter(|l| l.is_straight()).collect();

    // TODO
}

fn parse_vent_lines(lines: Vec<String>) -> Vec<Line> {
    let mut output_lines = vec![];

    for line in lines.iter() {
        let points: Vec<&str> = line.trim().split(" -> ").collect();

        if points.len() != 2 { panic!("invalid number of points on a line"); }

        let source_point: Vec<usize> = points[0].split(',').map(|p| p.parse().unwrap()).collect();
        let target_point: Vec<usize> = points[1].split(',').map(|p| p.parse().unwrap()).collect();

        output_lines.push(Line(Point(source_point[0], source_point[1]), Point(target_point[0], target_point[1])));
    }

    output_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "0,9 -> 5,9\n\
                                   8,0 -> 0,8\n\
                                   9,4 -> 3,4\n\
                                   2,2 -> 2,1\n\
                                   7,0 -> 7,4\n\
                                   6,4 -> 2,0\n\
                                   0,9 -> 2,9\n\
                                   3,4 -> 1,4\n\
                                   0,0 -> 8,8\n\
                                   5,5 -> 8,2";

    #[test]
    fn test_line_parser() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let lines = parse_vent_lines(input);

        let expected_lines = vec![
            Line(Point(0, 9), Point(5, 9)),
            Line(Point(8, 0), Point(0, 8)),
            Line(Point(9, 4), Point(3, 4)),
            Line(Point(2, 2), Point(2, 1)),
            Line(Point(7, 0), Point(7, 4)),
            Line(Point(6, 4), Point(2, 0)),
            Line(Point(0, 9), Point(2, 9)),
            Line(Point(3, 4), Point(1, 4)),
            Line(Point(0, 0), Point(8, 8)),
            Line(Point(5, 5), Point(8, 2)),
        ];

        assert_eq!(lines, expected_lines);
    }

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let lines: Vec<Line> = parse_vent_lines(input).into_iter().filter(|l| l.is_straight()).collect();

        // TODO
    }

    #[test]
    fn test_straight_lines() {
        let line = Line(Point(2, 7), Point(4, 7));
        assert!(line.is_horizontal());
        assert!(!line.is_vertical());
        assert!(line.is_straight());

        let line = Line(Point(2, 7), Point(2, 100));
        assert!(!line.is_horizontal());
        assert!(line.is_vertical());
        assert!(line.is_straight());

        let line = Line(Point(2, 7), Point(1, 10));
        assert!(!line.is_horizontal());
        assert!(!line.is_vertical());
        assert!(!line.is_straight());
    }
}
