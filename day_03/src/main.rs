use common::read_puzzle_input;

fn calculate_epsilon(entries: &Vec<isize>, bit_width: usize) -> usize {
    0
}

fn calculate_gamma(entries: &Vec<isize>, bit_width: usize) -> usize {
    0
}

fn diagnostic_power_level(input: &Vec<String>) -> usize {
    let bit_len = input[0].trim().len();

    let entries: Vec<isize> = input.iter().map(|bits| isize::from_str_radix(bits, 2).unwrap()).collect();

    let epsilon = calculate_epsilon(&entries, bit_len);
    let gamma = calculate_gamma(&entries, bit_len);

    epsilon * gamma
}

fn main() {
    let input_entries = read_puzzle_input(3);

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
}
