use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};

use crate::instrument::instrument;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
    id: usize,
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.trim().split(':');
        let id = s
            .next()
            .ok_or_else(|| anyhow!("Invalid card: no ':' found"))?;
        let id = id
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("Invalid card: no number found"))?
            .parse()?;

        let mut nums = s
            .next()
            .ok_or_else(|| anyhow!("Invalid card: nothing after ':' found"))?
            .split('|');
        let first_nums = nums
            .next()
            .ok_or_else(|| anyhow!("Invalid card: no '|' found"))?
            .trim();

        let winning_numbers = first_nums
            .split_whitespace()
            .map(|num| Ok(num.parse()?))
            .collect::<Result<HashSet<u8>>>()?;
        let second_nums = nums
            .next()
            .ok_or_else(|| anyhow!("Invalid card: nothing after '|' found"))?
            .trim();
        let numbers = second_nums
            .split_whitespace()
            .map(|num| Ok(num.parse()?))
            .collect::<Result<HashSet<u8>>>()?;
        Ok(Self {
            winning_numbers,
            numbers,
            id,
        })
    }
}

impl Card {
    fn score(&self) -> u32 {
        let winning = self.winning_numbers.intersection(&self.numbers).count() as u32;
        if winning == 0 {
            0
        } else {
            2_u32.pow(winning.saturating_sub(1))
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let mut sum = 0;
    for card in input.trim().split('\n').map(|card| card.parse::<Card>()) {
        let card = card?;
        sum += card.score();
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    unimplemented!()
}

pub fn solve() -> Result<()> {
    println!("\nDay 04");
    instrument!(part1(INPUT)?, part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 0);
    }
}
