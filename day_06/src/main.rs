#![allow(unused_variables)]

use common::read_puzzle_input;

#[derive(Debug)]
struct Pond {
    fish_buckets: [usize; 9]
}

impl Pond {
    fn advance(&mut self, count: usize) {
        for i in 0..count {
            let mut new_fish_buckets: [usize; 9] = [0; 9];
            for (ticks, count) in self.fish_buckets.iter().enumerate() {
                if ticks == 0 {
                    new_fish_buckets[6] += count;
                    new_fish_buckets[8] += count;
                } else {
                    new_fish_buckets[ticks - 1] += count;
                }
            }

            self.fish_buckets = new_fish_buckets;
        }
    }

    fn count(&self) -> usize {
        self.fish_buckets.iter().sum()
    }
}

impl From<String> for Pond {
    fn from(start: String) -> Self {
        let fish: Vec<usize> = start.trim().split(',').map(|i| i.parse::<usize>().unwrap()).collect();

        let mut pond = Pond { fish_buckets: [0; 9] };
        for f in fish.iter() {
            pond.fish_buckets[*f] += 1;
        }

        pond
    }
}

fn main() {
    let input = read_puzzle_input(6);
    let mut pond = Pond::from(input[0].clone());

    pond.advance(80);
    println!("first question:{}", pond.count());

    pond.advance(176);
    println!("second question:{}", pond.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let mut pond = Pond::from(input[0].clone());

        pond.advance(18);
        assert_eq!(pond.count(), 26);

        pond.advance(62);
        assert_eq!(pond.count(), 5934);
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let mut pond = Pond::from(input[0].clone());

        pond.advance(256);
        assert_eq!(pond.count(), 26984457539);
    }
}
