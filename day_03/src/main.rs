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

    let threshold = entries.len() / 2;
    let mut total = 0;

    for (bit, count) in bit_counts.iter().enumerate() {
        if *count <= threshold {
            total += 1 << (bit_width - bit - 1);
        }
    }

    total
}

fn calculate_gamma(entries: &Vec<isize>, bit_width: usize) -> usize {
    let bit_counts = bit_counts(entries, bit_width);

    let threshold = entries.len() / 2;
    let mut total = 0;

    for (bit, count) in bit_counts.iter().enumerate() {
        if *count > threshold {
            total += 1 << (bit_width - bit - 1);
        }
    }

    total
}

fn co2_scrubber_rating(entries: &Vec<isize>, bit_width: usize) -> usize {
    0
}

fn diagnostic_power_level(entries: &Vec<isize>, bit_width: usize) -> usize {
    let epsilon = calculate_epsilon(&entries, bit_width);
    let gamma = calculate_gamma(&entries, bit_width);

    epsilon * gamma
}

fn life_support_rating(entries: &Vec<isize>, bit_width: usize) -> usize {
    let co2 = co2_scrubber_rating(&entries, bit_width);
    let oxygen = oxygen_generator_rating(&entries, bit_width);

    co2 * oxygen
}

fn oxygen_generator_rating(entries: &Vec<isize>, bit_width: usize) -> usize {
    0
}

fn parse_entries(input: &Vec<String>) -> Vec<isize> {
    input.iter().map(|bits| isize::from_str_radix(bits, 2).unwrap()).collect()
}

fn main() {
    let input = read_puzzle_input(3);
    let parsed_input = parse_entries(&input);

    let bit_len = input[0].trim().len();

    println!("diagnostic power level: {}", diagnostic_power_level(&parsed_input, bit_len));
    println!("life support rating: {}", life_support_rating(&parsed_input, bit_len));
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

    #[test]
    fn test_second_challenge() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let entries = parse_entries(&input);

        assert_eq!(co2_scrubber_rating(&entries, 5), 10);
        assert_eq!(oxygen_generator_rating(&entries, 5), 23);

        assert_eq!(life_support_rating(&entries, 5), 230);
    }

    #[test]
    fn test_first_challenge() {
        let input: Vec<String> = REFERENCE_INPUT.lines().map(|e| e.to_string()).collect();
        let entries = parse_entries(&input);

        assert_eq!(calculate_epsilon(&entries, 5), 9);
        assert_eq!(calculate_gamma(&entries, 5), 22);

        assert_eq!(diagnostic_power_level(&entries, 5), 198);
    }
}
