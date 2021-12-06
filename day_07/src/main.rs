#![allow(unused_variables)]

use common::read_puzzle_input;

fn main() {
    let input_entries = read_puzzle_input(6);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "";

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
    }
}
