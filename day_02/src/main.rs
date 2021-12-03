use common::read_puzzle_input;

#[derive(Debug, PartialEq)]
enum Direction {
    Down(isize),
    Forward(isize),
    Up(isize),
}

impl TryFrom<String> for Direction {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let components: Vec<&str> = value.trim().split(' ').collect();

        if components.len() != 2 {
            return Err(());
        }

        let count = match components[1].parse::<isize>() {
            Ok(cnt) => cnt,
            Err(_) => {
                return Err(());
            }
        };

        match components[0] {
            "down" => Ok(Direction::Down(count)),
            "forward" => Ok(Direction::Forward(count)),
            "up" => Ok(Direction::Up(count)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Position {
}

fn main() {
    let input_entries = read_puzzle_input(2);
    let directions: Vec<Direction> = input_entries.into_iter().map(|e| Direction::try_from(e).unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_INPUT: &str = "forward 5\n\
                                   down 5\n\
                                   forward 8\n\
                                   up 3\n\
                                   down 8\n\
                                   forward 2";

    #[test]
    fn test_direction_conversion() {
        assert_eq!(Direction::try_from("down 17".to_string()), Ok(Direction::Down(17)));
        assert_eq!(Direction::try_from("forward 8".to_string()), Ok(Direction::Forward(8)));
        assert_eq!(Direction::try_from("up 70".to_string()), Ok(Direction::Up(70)));
    }

    #[test]
    fn test_run_sample_data() {
        let directions = REFERENCE_INPUT.iter().map(|e| Direction::try_from(e.to_string()).unwrap()).collect();
    }
}
