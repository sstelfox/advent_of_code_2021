use std::fs::File;
use std::io::Read;

fn count_sequential_increases(list: &[isize]) -> usize {
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

fn main() {
    let mut in_dat_fh = File::open("./data/day_01.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let input_entries: Vec<isize> = in_dat.lines().map(|i| i.parse::<isize>().unwrap()).collect();

    println!("{}", count_sequential_increases(&input_entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_in_data() {
        let sample_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_sequential_increases(&sample_data), 7);
    }
}
