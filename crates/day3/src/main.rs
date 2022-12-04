use itertools::Itertools;
use std::{
    collections::HashSet,
    env,
    io::{self, prelude::*},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("unexpected item"),
    }
}

fn part1(reader: impl BufRead) -> Result<u32> {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let mut chars: Vec<_> = line.chars().collect();
        let other = chars.split_off(chars.len() / 2);
        let comp1: HashSet<_> = chars.iter().collect();
        let comp2: HashSet<_> = other.iter().collect();
        let diff = comp1.intersection(&comp2).next().unwrap();
        sum += priority(**diff);
    }
    Ok(sum)
}

fn part2(reader: impl BufRead) -> Result<u32> {
    let mut sum = 0;
    for chunk in &reader.lines().chunks(3) {
        let mut sets: Vec<HashSet<_>> = Vec::new();
        for line in chunk {
            let line = line?;
            sets.push(line.chars().collect());
        }
        let mut iter = sets.into_iter();
        let intersection = iter
            .next()
            .map(|set| iter.fold(set, |set1, set2| &set1 & &set2))
            .unwrap();
        let intersection = intersection.iter().next().unwrap();
        sum += priority(*intersection);
    }
    Ok(sum)
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
        let expected = 157;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_puzzle_input() {
        let actual = part1(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 7875;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example_input() {
        let actual = part2(EXAMPLE_INPUT.as_bytes()).unwrap();
        let expected = 70;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_puzzle_input() {
        let actual = part2(PUZZLE_INPUT.as_bytes()).unwrap();
        let expected = 2479;
        assert_eq!(actual, expected);
    }
}
