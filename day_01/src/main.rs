use std::fs::File;
use std::io::Read;

fn count_sequential_increases(list: &[isize]) -> usize {
    debug_assert!(list.len() > 0);

    let mut list_iter = list.iter();
    let mut seq_increase_count = 0;

    let mut last_entry = list_iter.next().unwrap();
    for entry in list_iter {
        if entry > last_entry {
            seq_increase_count += 1;
        }

        last_entry = entry;
    }

    seq_increase_count
}

fn count_sliding_sequential_increases(slide_size: usize, list: &[isize]) -> usize {
    debug_assert!(list.len() > (slide_size - 1));

    let sublist = &list[0..slide_size];

    let mut last_sum: isize = sublist.iter().sum();
    let mut seq_increase_count = 0;

    for offset in 1..(list.len() - slide_size + 1) {
        let sublist = &list[offset..(offset + slide_size)];

        if sublist.len() < slide_size {
            panic!("sublist wasn't large enough");
        }

        let current_sum: isize = sublist.iter().sum();

        if current_sum > last_sum {
            seq_increase_count += 1;
        }

        last_sum = current_sum;
    }

    seq_increase_count
}

fn main() {
    let mut in_dat_fh = File::open("./data/day_01.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let input_entries: Vec<isize> = in_dat.lines().map(|i| i.parse::<isize>().unwrap()).collect();

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
