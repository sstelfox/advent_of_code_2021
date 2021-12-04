use common::read_puzzle_input;

fn bit_counts(entries: &Vec<isize>, bit_width: usize) -> Vec<usize> {
    let mut bit_counts = vec![0; bit_width];

    for ent in entries.iter() {
        for bit in 0..bit_width {
            let relevant_bit = 1 << bit;

            if (*ent & relevant_bit) > 0 {
                let bit_target = (bit_width - bit) - 1;
                bit_counts[bit_target] += 1;
            }
        }
    }

    bit_counts
}

fn calculate_epsilon(entries: &Vec<isize>, bit_width: usize) -> usize {
    let bit_counts = bit_counts(entries, bit_width);

    let length = entries.len();
    let threshold = length / 2;
    let mut total = 0;

    for (bit, count) in bit_counts.iter().enumerate() {
        if *count <= threshold {
            total += 1 << bit;
        }
    }

    total
}

fn calculate_gamma(entries: &Vec<isize>, bit_width: usize) -> usize {
    let bit_counts = bit_counts(entries, bit_width);

    let length = entries.len();
    let threshold = length / 2;
    let mut total = 0;

    for (bit, count) in bit_counts.iter().enumerate() {
        if *count > threshold {
            total += 1 << bit;
        }
    }

    println!("total:{:#01$b}", total, bit_width);

    total
}

fn diagnostic_power_level(input: &Vec<String>) -> usize {
    let bit_len = input[0].trim().len();

    let entries: Vec<isize> = input.iter().map(|bits| isize::from_str_radix(bits, 2).unwrap()).collect();

    let epsilon = calculate_epsilon(&entries, bit_len);
    let gamma = calculate_gamma(&entries, bit_len);

    epsilon * gamma
}

fn main() {
    let _input_entries = read_puzzle_input(3);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "00100\n\
                                   11110\n\
                                   10110\n\
                                   10111\n\
                                   10101\n\
                                   01111\n\
                                   00111\n\
                                   11100\n\
                                   10000\n\
                                   11001\n\
                                   00010\n\
                                   01010";

    #[test]
    fn test_first_input() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();

        let entries: Vec<isize> = input.iter().map(|bits| isize::from_str_radix(bits, 2).unwrap()).collect();
        assert_eq!(calculate_epsilon(&entries, 5), 9);
        assert_eq!(calculate_gamma(&entries, 5), 22);

        assert_eq!(diagnostic_power_level(&input), 198);
    }

    #[test]
    fn test_bit_counts() {
        let input = vec![0b101010];
        assert_eq!(bit_counts(&input, 6), vec![1, 0, 1, 0, 1, 0]);

        let input = vec![
            0b01010101,
            0b11110000,
            0b00001111,
        ];
        assert_eq!(bit_counts(&input, 8), vec![1, 2, 1, 2, 1, 2, 1, 2]);
    }
}
