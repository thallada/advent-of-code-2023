use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::instrument::instrument;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug)]
struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Handful {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for handful in s.split(',') {
            let mut handful = handful.trim().split(' ');
            let num = handful
                .next()
                .ok_or_else(|| anyhow!("Invalid handful"))?
                .parse()?;
            match handful.next().ok_or_else(|| anyhow!("Invalid handful"))? {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => return Err(anyhow!("Invalid handful")),
            }
        }
        Ok(Self { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.trim().split(':');
        let id = s
            .next()
            .ok_or_else(|| anyhow!("Invalid game line: no ':' found"))?;
        let id = id
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("Invalid game line: no number found"))?
            .parse()?;
        let handfuls = s
            .next()
            .ok_or_else(|| anyhow!("Invalid game line: nothing after ':' found"))?;
        let handfuls = handfuls
            .split(';')
            .map(|handful| handful.parse())
            .collect::<Result<Vec<Handful>>>()?;
        Ok(Self { id, handfuls })
    }
}

fn part1(input: &str) -> Result<u32> {
    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let games = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Game>>>()?;
    Ok(games
        .into_iter()
        .filter(|game| {
            !game.handfuls.iter().any(|handful| {
                handful.red > total_red || handful.green > total_green || handful.blue > total_blue
            })
        })
        .map(|game| game.id)
        .sum())
}

fn part2(input: &str) -> Result<u32> {
    let games = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Game>>>()?;
    let mut sum = 0;
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for handful in game.handfuls {
            if handful.red > max_red {
                max_red = handful.red;
            }
            if handful.green > max_green {
                max_green = handful.green;
            }
            if handful.blue > max_blue {
                max_blue = handful.blue;
            }
        }
        sum += max_red * max_green * max_blue;
    }
    Ok(sum)
}

pub fn solve() -> Result<()> {
    println!("\nDay 02");
    instrument!(part1(INPUT)?, part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 2286);
    }
}
