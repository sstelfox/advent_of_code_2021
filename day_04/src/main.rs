#![allow(unused_variables)]

use common::read_puzzle_input;

struct Board {
    numbers: [[(usize, bool); 5]; 5],
}

impl From<&[String]> for Board {
    fn from(input: &[String]) -> Board {
        let board_lines: Vec<&str> = input[1..].iter().map(|l| l.trim()).collect();

        let board_numbers: Vec<Vec<usize>> = board_lines.iter()
                .map(|line| {
                    line.split(' ')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

        println!( "{:?}", board_numbers);

        Board {
            numbers: [[(0, false); 5]; 5],
        }
    }
}

fn main() {
    let input = read_puzzle_input(4);
    let rng_nums = parse_random_header(&input[0]);

    let mut board_list = vec![];
    for board in input[1..].chunks(6) {
        board_list.push(parse_board(board));
    }
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
    fn test_board_parser() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        let mut board_list = vec![];
        for board in input[1..].chunks(6) {
            board_list.push(parse_board(board));
        }
    }
}
