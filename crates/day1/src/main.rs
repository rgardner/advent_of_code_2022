use std::{
    cmp, env,
    io::{self, prelude::*},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(reader: impl BufRead) -> Result<i32> {
    let mut max = 0;
    let mut curr = 0;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            max = cmp::max(curr, max);
            curr = 0;
            continue;
        }
        curr += line.parse::<i32>()?;
    }

    Ok(cmp::max(curr, max))
}

/// Pushes val into the max sorted slice, removing smallest element.
///
/// Invariant: `slice` is sorted in descending order.
fn push_max_sorted(val: i32, slice: &mut [i32]) {
    for i in 0..slice.len() {
        if val > slice[i] {
            slice[i..].rotate_right(1);
            slice[i] = val;
            break;
        }
    }
}

fn part2(reader: impl BufRead) -> Result<i32> {
    let mut top = [0, 0, 0];
    let mut curr = 0;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            push_max_sorted(curr, &mut top);
            curr = 0;
            continue;
        }
        curr += line.parse::<i32>()?;
    }
    push_max_sorted(curr, &mut top);

    Ok(top.iter().sum())
}

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("1") => println!("{}", part1(io::stdin().lock())?),
        Some("2") => println!("{}", part2(io::stdin().lock())?),
        x => panic!("invalid argument: {x:?}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");
    static PUZZLE_INPUT: &str = include_str!("../data/puzzle_input.txt");

    #[test]
    fn test_part1_example_input() {
        let actual = part1(EXAMPLE_INPUT.as_bytes()).unwrap();
        let expected = 24_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_puzzle_input() {
        let actual = part1(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 66_306;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example_input() {
        let actual = part2(EXAMPLE_INPUT.as_bytes()).unwrap();
        let expected = 45_000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_puzzle_input() {
        let actual = part2(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 195_292;
        assert_eq!(actual, expected);
    }
}
