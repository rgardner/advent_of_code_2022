use std::{
    env,
    io::{self, prelude::*},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn shape_score(my_shape: char) -> u32 {
    match my_shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("invalid shape"),
    }
}

fn round_outcome_score(theirs: char, mine: char) -> u32 {
    let theirs = theirs as u32 - 'A' as u32;
    let mine = mine as u32 - 'X' as u32;
    match (theirs, mine) {
        (t, m) if t == m => 3, // tie
        (0, 2) => 0,           // lose special case: wrap around
        (2, 0) => 6,           // win special case: wrap around
        (t, m) if m > t => 6,  // win
        _ => 0,                // lose
    }
}

fn part1(reader: impl BufRead) -> Result<u32> {
    let mut total = 0;
    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();
        let theirs = chars.next().unwrap();
        chars.next(); // skip space
        let mine = chars.next().unwrap();
        total += shape_score(mine) + round_outcome_score(theirs, mine);
    }

    Ok(total)
}

fn round_outcome_score_part2(outcome: char) -> u32 {
    match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("unknown outcome"),
    }
}

fn part2(reader: impl BufRead) -> Result<u32> {
    let mut total = 0;
    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();
        let theirs = chars.next().unwrap();
        chars.next(); // skip space
        let outcome = chars.next().unwrap();

        let shapes = ['Z', 'X', 'Y', 'Z', 'X'];
        let shape_idx: usize = ((theirs as u32 - 'A' as u32) + (outcome as u32 - 'X' as u32))
            .try_into()
            .unwrap();
        total += shape_score(shapes[shape_idx]) + round_outcome_score_part2(outcome);
    }

    Ok(total)
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
        let expected = 15;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_puzzle_input() {
        let actual = part1(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 12_645;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example_input() {
        let actual = part2(EXAMPLE_INPUT.as_bytes()).unwrap();
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_puzzle_input() {
        let actual = part2(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 11_756;
        assert_eq!(actual, expected);
    }
}
