#![allow(unused_variables)]

use common::read_puzzle_input;

#[derive(Debug)]
struct Pond {
    fish: Vec<usize>,
}

impl Pond {
    fn advance(&mut self, count: usize) {
        for _ in 0..count {
            let mut new_fish = 0;

            for fish in self.fish.iter_mut() {
                if *fish == 0 {
                    *fish += 6;
                    new_fish += 1;
                } else {
                    *fish -= 1;
                }
            }

            for _ in 0..new_fish {
                self.fish.push(8);
            }
        }
    }

    fn count(&self) -> usize {
        self.fish.len()
    }
}

fn main() {
    let input = read_puzzle_input(6);
    let fish: Vec<usize> = input[0].trim().split(',').map(|i| i.parse::<usize>().unwrap()).collect();
    let mut pond = Pond { fish };

    pond.advance(80);
    println!("first question:{}", pond.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let fish: Vec<usize> = input[0].trim().split(',').map(|i| i.parse::<usize>().unwrap()).collect();
        let mut pond = Pond { fish };

        pond.advance(18);
        assert_eq!(pond.count(), 26);

        pond.advance(62);
        assert_eq!(pond.count(), 5934);
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        // TODO
    }
}
