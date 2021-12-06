#![allow(unused_variables)]

use common::read_puzzle_input;

#[derive(Debug, PartialEq)]
struct Board {
    positions: Vec<Vec<usize>>,
}

impl Board {
    fn mark_line(&mut self, line: &Line) {
        if line.slope().is_infinite() {
            let x_pos = line.left().0;
            for y_pos in line.bottom().1..=line.top().1 {
                self.positions[y_pos][x_pos] += 1;
            }
        } else {
            for x_pos in line.left().0..=line.right().0 {
                let y_pos = line.solve_for_y(x_pos);
                self.positions[y_pos][x_pos] += 1;
            }
        }
    }

    fn overlapping_position_count(&self) -> usize {
        let mut count = 0;

        for row in self.positions.iter() {
            for position in row.iter() {
                if *position > 1 {
                    count += 1;
                }
            }
        }

        count
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.positions.iter() {
            for position in row.iter() {
                if *position == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", position)?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl From<(usize, usize)> for Board {
    fn from((width, height): (usize, usize)) -> Self {
        Board {
            positions: vec![vec![0; width]; height],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn bottom(&self) -> &Point {
        if self.0.1 >= self.1.1 {
            &self.1
        } else {
            &self.0
        }
    }

    fn left(&self) -> &Point {
        if self.0.0 >= self.1.0 {
            &self.1
        } else {
            &self.0
        }
    }

    fn right(&self) -> &Point {
        if self.0.0 >= self.1.0 {
            &self.0
        } else {
            &self.1
        }
    }

    fn top(&self) -> &Point {
        if self.0.1 >= self.1.1 {
            &self.0
        } else {
            &self.1
        }
    }
}

impl Line {
    fn solve_for_y(&self, x: usize) -> usize {
        let y_value = self.slope() * x as f64 + self.y_intercept() as f64;
        y_value.round() as usize
    }

    fn slope(&self) -> f64 {
        (self.right().1 as f64 - self.left().1 as f64) / (self.right().0 as f64 - self.left().0 as f64)
    }

    fn y_intercept(&self) -> f64 {
        -1.0 * ((self.slope() * self.left().0 as f64) - self.left().1 as f64)
    }
}

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

fn board_size(lines: &Vec<Line>) -> (usize, usize) {
    (max_width(lines) + 1, max_height(lines) + 1)
}

fn main() {
    let input_entries = read_puzzle_input(5);

    let second_lines: Vec<Line> = parse_vent_lines(input_entries).into_iter().collect();
    let mut second_board = Board::from(board_size(&second_lines));
    for line in second_lines.iter() {
        second_board.mark_line(line);
    }

    let first_lines: Vec<Line> = second_lines.into_iter().filter(|l| l.is_straight()).collect();
    let mut first_board = Board::from(board_size(&first_lines));
    for line in first_lines.iter() {
        first_board.mark_line(line);
    }

    println!("first answer: {}", first_board.overlapping_position_count());
    println!("second answer: {}", second_board.overlapping_position_count());
}

fn max_height(lines: &Vec<Line>) -> usize {
    let mut max_height = 0;

    for l in lines.iter() {
        if l.0.1 > max_height {
            max_height = l.0.1;
        }

        if l.1.1 > max_height {
            max_height = l.1.1;
        }
    }

    max_height
}

fn max_width(lines: &Vec<Line>) -> usize {
    let mut max_width = 0;

    for l in lines.iter() {
        if l.0.0 > max_width {
            max_width = l.0.0;
        }

        if l.1.0 > max_width {
            max_width = l.1.0;
        }
    }

    max_width
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
    fn test_board_size() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let lines = parse_vent_lines(input);

        assert_eq!(board_size(&lines), (10, 10));
    }

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
    fn test_line_solutions() {
        let line = Line(Point(3, 5), Point(6, 11));
        assert_eq!(line.slope(), 2.0);
        assert_eq!(line.y_intercept(), -1.0);
        assert_eq!(line.solve_for_y(9), 17);
    }

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let lines: Vec<Line> = parse_vent_lines(input).into_iter().filter(|l| l.is_straight()).collect();

        let mut board = Board::from(board_size(&lines));
        for line in lines.iter() {
            board.mark_line(line);
        }

        assert_eq!(board.overlapping_position_count(), 5);
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let lines: Vec<Line> = parse_vent_lines(input).into_iter().collect();

        let mut board = Board::from(board_size(&lines));
        for line in lines.iter() {
            board.mark_line(line);
        }

        assert_eq!(board.overlapping_position_count(), 12);
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
