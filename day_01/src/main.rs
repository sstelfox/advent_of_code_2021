use common::read_puzzle_input;

fn count_sequential_increases(list: &[isize]) -> usize {
    count_sliding_sequential_increases(1, list)
}

fn count_sliding_sequential_increases(slide_size: usize, list: &[isize]) -> usize {
    let sums: Vec<isize> = list
        .windows(slide_size)
        .map(|window| window.iter().sum())
        .collect();

    sums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn main() {
    let input_entries: Vec<isize> = read_puzzle_input(1)
        .iter()
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    println!("{}", count_sequential_increases(&input_entries));
    println!("{}", count_sliding_sequential_increases(3, &input_entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_in_data() {
        let sample_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_sequential_increases(&sample_data), 7);
    }

    #[test]
    fn test_count_sliding_sequential_increases_in_data() {
        let sample_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_sliding_sequential_increases(3, &sample_data), 5);
    }
}
