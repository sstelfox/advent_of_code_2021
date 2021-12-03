use std::fs::File;
use std::io::Read;

pub fn read_puzzle_input(day_count: usize) -> Vec<String> {
    let file_name = format!("./data/day_{:02}.txt", day_count);

    let mut file_handle = File::open(file_name).unwrap();
    let mut data = String::new();

    file_handle.read_to_string(&mut data).unwrap();

    data.lines().map(|s| s.to_string()).collect()
}
