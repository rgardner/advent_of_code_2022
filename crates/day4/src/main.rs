use std::{
    env,
    io::{self, prelude::*},
    ops::Range,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait RangeExt<Idx> {
    fn contains_range(&self, other: &Range<Idx>) -> bool;
    fn overlap(&self, other: &Range<Idx>) -> bool;
}

impl<Idx> RangeExt<Idx> for Range<Idx>
where
    Idx: PartialOrd<Idx>,
{
    fn contains_range(&self, other: &Range<Idx>) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlap(&self, other: &Range<Idx>) -> bool {
        self.start < other.end && other.start < self.end
    }
}

fn part1(reader: impl BufRead) -> Result<u32> {
    let mut num_fully_contained = 0;
    for line in reader.lines() {
        let line = line?;
        let ranges: Vec<Vec<i32>> = line
            .split(',')
            .map(|r| r.split('-').map(|s| s.parse::<i32>().unwrap()).collect())
            .collect();
        let r1 = Range {
            start: ranges[0][0],
            end: ranges[0][1] + 1,
        };
        let r2 = Range {
            start: ranges[1][0],
            end: ranges[1][1] + 1,
        };
        if r1.contains_range(&r2) || r2.contains_range(&r1) {
            num_fully_contained += 1;
        }
    }
    Ok(num_fully_contained)
}

fn part2(reader: impl BufRead) -> Result<u32> {
    let mut num_overlap = 0;
    for line in reader.lines() {
        let line = line?;
        let ranges: Vec<Vec<i32>> = line
            .split(',')
            .map(|r| r.split('-').map(|s| s.parse::<i32>().unwrap()).collect())
            .collect();
        let r1 = Range {
            start: ranges[0][0],
            end: ranges[0][1] + 1,
        };
        let r2 = Range {
            start: ranges[1][0],
            end: ranges[1][1] + 1,
        };
        if r1.overlap(&r2) {
            num_overlap += 1;
        }
    }
    Ok(num_overlap)
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
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_puzzle_input() {
        let actual = part1(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 599;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example_input() {
        let actual = part2(EXAMPLE_INPUT.as_bytes()).unwrap();
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_puzzle_input() {
        let actual = part2(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 928;
        assert_eq!(actual, expected);
    }
}
