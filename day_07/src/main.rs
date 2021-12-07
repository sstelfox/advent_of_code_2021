#![allow(unused_variables)]

use common::read_puzzle_input;

struct ShipLine {
    positions: Vec<usize>,
    width: usize,
}

fn factorial_sum(count: usize) -> usize {
    let mut sum = 0;

    for n in 0..=count {
        sum += n;
    }

    sum
}

impl ShipLine {
    fn factorial_fuel_to_position(&self, target_position: usize) -> usize {
        if target_position >= self.width {
            panic!("attempt to move ships into position beyond the end");
        }

        let mut total_fuel = 0;

        for (existing_position, count) in self.positions.iter().enumerate() {
            let distance = (target_position as isize - existing_position as isize).abs() as usize;

            if distance == 0 {
                continue;
            }

            let fuel_cost = factorial_sum(distance);
            total_fuel += fuel_cost * count;
        }

        total_fuel
    }

    fn minimum_factorial_fuel_usage(&self) -> (usize, usize) {
        let mut min_fuel: Option<usize> = None;
        let mut target_position: usize = 0;

        for target in 0..self.width {
            let fuel_for_target  = self.factorial_fuel_to_position(target);

            if let Some(current_val) = min_fuel {
                if fuel_for_target < current_val {
                    target_position = target;
                    min_fuel = Some(fuel_for_target);
                }
            } else {
                target_position = target;
                min_fuel = Some(fuel_for_target);
            };
        }

        (target_position, min_fuel.unwrap())
    }

    fn minimum_linear_fuel_usage(&self) -> (usize, usize) {
        let mut min_fuel: Option<usize> = None;
        let mut target_position: usize = 0;

        for target in 0..self.width {
            let fuel_for_target  = self.linear_fuel_to_position(target);

            if let Some(current_val) = min_fuel {
                if fuel_for_target < current_val {
                    target_position = target;
                    min_fuel = Some(fuel_for_target);
                }
            } else {
                target_position = target;
                min_fuel = Some(fuel_for_target);
            };
        }

        (target_position, min_fuel.unwrap())
    }

    fn linear_fuel_to_position(&self, target_position: usize) -> usize {
        if target_position >= self.width {
            panic!("attempt to move ships into position beyond the end");
        }

        let mut total_fuel = 0;

        for (existing_position, count) in self.positions.iter().enumerate() {
            let distance = (target_position as isize - existing_position as isize).abs() as usize;
            total_fuel += distance * count;
        }

        total_fuel
    }
}

impl From<Vec<usize>> for ShipLine {
    fn from(pos_list: Vec<usize>) -> Self {
        let width: usize = *pos_list.iter().max().unwrap() + 1;
        let mut positions: Vec<usize> = vec![0; width];

        for pos in pos_list.into_iter() {
            positions[pos] += 1;
        }

        Self { positions, width }
    }
}

fn main() {
    let input_entries = read_puzzle_input(7);
    let positions: Vec<usize> = input_entries[0].split(",").map(|i| i.parse::<usize>().unwrap()).collect();

    let ship_line = ShipLine::from(positions);
    println!("minimum fuel for linear alignment: {:?}", ship_line.minimum_linear_fuel_usage());
    println!("minimum fuel for factorial alignment: {:?}", ship_line.minimum_factorial_fuel_usage());
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_first_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let positions: Vec<usize> = input[0].split(',').map(|i| i.parse::<usize>().unwrap()).collect();

        assert_eq!(positions.len(), 10);

        let ship_line = ShipLine::from(positions);

        assert_eq!(ship_line.linear_fuel_to_position(1), 41);
        assert_eq!(ship_line.linear_fuel_to_position(2), 37);
        assert_eq!(ship_line.linear_fuel_to_position(3), 39);
        assert_eq!(ship_line.linear_fuel_to_position(10), 71);

        assert_eq!(ship_line.minimum_linear_fuel_usage(), (2, 37));
    }

    #[test]
    fn test_second_part() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let positions: Vec<usize> = input[0].split(',').map(|i| i.parse::<usize>().unwrap()).collect();

        let ship_line = ShipLine::from(positions);

        assert_eq!(ship_line.factorial_fuel_to_position(2), 206);
        assert_eq!(ship_line.factorial_fuel_to_position(5), 168);

        assert_eq!(ship_line.minimum_factorial_fuel_usage(), (5, 168));
    }
}
