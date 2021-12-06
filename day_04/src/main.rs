#![allow(unused_variables)]

use common::read_puzzle_input;

#[derive(Clone, Debug, PartialEq)]
struct Board {
    numbers: [(usize, bool); 25],
}

impl Board {
    fn is_solved(&self) -> bool {
        // Check rows
        if self.numbers.chunks(5).any(|row| row.iter().all(|(_, mark)| *mark)) {
            return true;
        }

        // Check columns
        for column in 0..5 {
            if self.numbers.chunks(5).all(|row| row[column].1) {
                return true;
            }
        }

        false
    }

    fn mark_number(&mut self, number: usize) {
        for (_num, marked) in self.numbers.iter_mut().filter(|(n, _)| n == &number) {
            *marked = true;
        }
    }

    fn unmarked_score(&self) -> usize {
        self.numbers.iter().filter(|(_, marked)| !marked).map(|(num, _)| num).sum()
    }
}

impl From<&[String]> for Board {
    fn from(input: &[String]) -> Board {
        let board_lines: Vec<&str> = input[1..].iter().map(|l| l.trim()).collect();

        let raw_numbers: Vec<usize> = board_lines.iter()
                .map(|line| {
                    line.split(' ')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .flatten()
                .collect();

        let board_numbers: Vec<(usize, bool)> = raw_numbers.into_iter().map(|n| (n, false)).collect();
        let mut numbers: [(usize, bool); 25] =  Default::default();
        numbers.copy_from_slice(&board_numbers[..25]);

        Board { numbers }
    }
}

fn main() {
    let input = read_puzzle_input(4);
    let rng_nums = parse_random_header(&input[0]);

    let mut board_list = vec![];
    for board in input[1..].chunks(6) {
        board_list.push(parse_board(board));
    }

    let mut first_game = board_list.clone();
    let result = play_boards(&mut first_game, &rng_nums);
    println!("first part result: {:?}", result);

    let result = last_board(board_list, &rng_nums);
    println!("second part result: {:?}", result);
}

fn parse_board(lines: &[String]) -> Board {
    Board::from(lines)
}

fn parse_random_header(line: &str) -> Vec<usize> {
    line.trim()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn last_board(mut boards: Vec<Board>, call_outs: &Vec<usize>) -> Option<usize> {
    for number in call_outs.iter() {

        for board in boards.iter_mut() {
            board.mark_number(*number);
        }

        if boards.len() == 1 && boards[0].is_solved() {
            return Some(*number * boards[0].unmarked_score());
        } else {
            boards = boards.into_iter().filter(|b| !b.is_solved()).collect();
        }
    }

    None
}

fn play_boards(boards: &mut Vec<Board>, call_outs: &Vec<usize>) -> Option<usize> {
    for number in call_outs.iter() {
        for board in boards.iter_mut() {
            board.mark_number(*number);

            if board.is_solved() {
                return Some(*number * board.unmarked_score());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                                   \n\
                                   22 13 17 11  0\n\
                                    8  2 23  4 24\n\
                                   21  9 14 16  7\n\
                                    6 10  3 18  5\n\
                                    1 12 20 15 19\n\
                                   \n\
                                    3 15  0  2 22\n\
                                    9 18 13 17  5\n\
                                   19  8  7 25 23\n\
                                   20 11 10 24  4\n\
                                   14 21 16 12  6\n\
                                   \n\
                                   14 21 17 24  4\n\
                                   10 16 15  9 19\n\
                                   18  8 23 26 20\n\
                                   22 11 13  6  5\n\
                                    2  0 12  3  7";

    #[test]
    fn test_random_parser() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let rng_nums = parse_random_header(&input[0]);
        assert_eq!(
            rng_nums[..12],
            vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]
        );
    }

    #[test]
    fn test_reference_game() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        let rng_nums = parse_random_header(&input[0]);

        let mut board_list = vec![];
        for board in input[1..].chunks(6) {
            board_list.push(parse_board(board));
        }

        assert_eq!(play_boards(&mut board_list, &rng_nums), Some(4512));
    }

    #[test]
    fn test_second_answer() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        let rng_nums = parse_random_header(&input[0]);

        let mut board_list = vec![];
        for board in input[1..].chunks(6) {
            board_list.push(parse_board(board));
        }

        assert_eq!(last_board(board_list, &rng_nums), Some(1924));
    }

    #[test]
    fn test_board_parser() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        let mut board_list = vec![];
        for board in input[1..].chunks(6) {
            board_list.push(parse_board(board));
        }

        let expected_boards = vec![
            Board {
                numbers: [
                    (22, false), (13, false), (17, false), (11, false), (0, false),
                    (8, false),  (2, false),  (23, false), (4, false),  (24, false),
                    (21, false), (9, false),  (14, false), (16, false), (7, false),
                    (6, false),  (10, false), (3, false),  (18, false), (5, false),
                    (1, false),  (12, false), (20, false), (15, false), (19, false),
                ]
            },
            Board {
                numbers: [
                    (3, false),  (15, false), (0, false),  (2, false),  (22, false),
                    (9, false),  (18, false), (13, false), (17, false), (5, false),
                    (19, false), (8, false),  (7, false),  (25, false), (23, false),
                    (20, false), (11, false), (10, false), (24, false), (4, false),
                    (14, false), (21, false), (16, false), (12, false), (6, false),
                ]
            },
            Board {
                numbers: [
                    (14, false), (21, false), (17, false), (24, false), (4, false),
                    (10, false), (16, false), (15, false), (9, false),  (19, false),
                    (18, false), (8, false),  (23, false), (26, false), (20, false),
                    (22, false), (11, false), (13, false), (6, false),  (5, false),
                    (2, false),  (0, false),  (12, false), (3, false),  (7, false),
                ]
            },
        ];

        assert_eq!(board_list, expected_boards);
    }

    #[test]
    fn test_board_marking() {
        let mut board = Board {
            numbers: [
                (0, false), (1, false), (2, false), (3, false), (4, false),
                (5, false), (6, false), (7, false), (8, false),  (9, false),
                (10, false), (11, false),  (12, false), (13, false), (14, false),
                (15, false), (6, false), (13, false), (1, false),  (2, false),
                (24, false),  (0, false),  (12, false), (3, false),  (9, false),
            ]
        };

        board.mark_number(2);
        assert!(board.numbers[2].1);
        assert!(board.numbers[19].1);

        board.mark_number(24);
        assert!(board.numbers[20].1);

        let expected_board = Board {
            numbers: [
                (0, false), (1, false), (2, true), (3, false), (4, false),
                (5, false), (6, false), (7, false), (8, false),  (9, false),
                (10, false), (11, false),  (12, false), (13, false), (14, false),
                (15, false), (6, false), (13, false), (1, false),  (2, true),
                (24, true),  (0, false),  (12, false), (3, false),  (9, false),
            ]
        };

        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_board_scoring() {
        let board = Board {
            numbers: [
                (14, true), (21, true), (17, true), (24, true), (4, true),
                (10, false), (16, false), (15, false), (9, true),  (19, false),
                (18, false), (8, false),  (23, true), (26, false), (20, false),
                (22, false), (11, true), (13, false), (6, false),  (5, true),
                (2, true),  (0, true),  (12, false), (3, false),  (7, true),
            ]
        };

        assert_eq!(board.unmarked_score(), 188);
    }

    #[test]
    fn test_solution_verification() {
        let board = Board {
            numbers: [
                (14, false), (21, false), (17, false), (24, false), (4, false),
                (10, false), (16, false), (15, false), (9, false),  (19, false),
                (18, false), (8, false),  (23, false), (26, false), (20, false),
                (22, false), (11, false), (13, false), (6, false),  (5, false),
                (2, false),  (0, false),  (12, false), (3, false),  (7, false),
            ]
        };
        assert!(!board.is_solved());

        let board = Board {
            numbers: [
                (14, false), (21, false), (17, false), (24, false), (4, false),
                (10, true), (16, true), (15, true), (9, true),  (19, true),
                (18, false), (8, false),  (23, false), (26, false), (20, false),
                (22, false), (11, false), (13, false), (6, false),  (5, false),
                (2, false),  (0, false),  (12, false), (3, false),  (7, false),
            ]
        };
        assert!(board.is_solved());

        let board = Board {
            numbers: [
                (14, false), (21, false), (17, false), (24, false), (4, true),
                (10, false), (16, false), (15, false), (9, false),  (19, true),
                (18, false), (8, false),  (23, false), (26, false), (20, true),
                (22, false), (11, false), (13, false), (6, false),  (5, true),
                (2, false),  (0, false),  (12, false), (3, false),  (7, true),
            ]
        };
        assert!(board.is_solved());

        let board = Board {
            numbers: [
                (14, true), (21, false), (17, false), (24, false), (4, false),
                (10, false), (16, true), (15, false), (9, false),  (19, false),
                (18, false), (8, false),  (23, true), (26, false), (20, false),
                (22, false), (11, false), (13, false), (6, true),  (5, false),
                (2, false),  (0, false),  (12, false), (3, false),  (7, true),
            ]
        };
        assert!(!board.is_solved());
    }
}
