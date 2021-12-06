#![allow(unused_variables)]

use num_bigint::{BigUint, ToBigUint};

use common::read_puzzle_input;

fn create_empty_fish_buckets() -> [BigUint; 9] {
    [
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
        0_usize.to_biguint().unwrap(),
    ]
}

#[derive(Debug)]
struct Pond {
    fish_buckets: [BigUint; 9]
}

impl Default for Pond {
    fn default() -> Self {
        Pond { fish_buckets: create_empty_fish_buckets() }
    }
}

impl Pond {
    fn advance(&mut self, count: usize) {
        for i in 0..count {
            let mut new_fish_buckets = create_empty_fish_buckets();
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

    fn count(&self) -> BigUint {
        self.fish_buckets.iter().sum()
    }
}

impl From<String> for Pond {
    fn from(start: String) -> Self {
        let fish: Vec<usize> = start.trim().split(',').map(|i| i.parse::<usize>().unwrap()).collect();

        let mut pond = Pond::default();
        for f in fish.iter() {
            pond.fish_buckets[*f] += 1_usize.to_biguint().unwrap();
        }

        pond
    }
}

fn main() {
    //let input = read_puzzle_input(6);
    //let mut pond = Pond::from(input[0].clone());

    //pond.advance(80);
    //println!("first question:{}", pond.count());

    //pond.advance(176);
    //println!("second question:{}", pond.count());

    let mut pond = Pond::from("3,4,3,1,2".to_string());
    pond.advance(9999999);
    println!("9999999 challenge result: {}", pond.count());
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
        assert_eq!(pond.count(), 26_usize.to_biguint().unwrap());

        pond.advance(62);
        assert_eq!(pond.count(), 5934_usize.to_biguint().unwrap());
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let mut pond = Pond::from(input[0].clone());

        pond.advance(256);
        assert_eq!(pond.count(), 26984457539_usize.to_biguint().unwrap());
    }
}
