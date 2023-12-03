use std::str::FromStr;

use anyhow::Result;

use crate::instrument::instrument;

const INPUT: &str = include_str!("input/input.txt");

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Number {
    start: Coord,
    end: Coord,
    value: u32,
}

impl Number {
    fn adjacent_to(&self, other: &Coord) -> bool {
        other.x >= self.start.x.saturating_sub(1)
            && other.x <= self.end.x + 1
            && other.y >= self.start.y.saturating_sub(1)
            && other.y <= self.end.y + 1
    }
}

#[derive(Debug)]
struct Engine {
    symbols: Vec<Coord>,
    numbers: Vec<Number>,
}

impl FromStr for Engine {
    type Err = anyhow::Error;
    #[allow(unused_assignments)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols: Vec<Coord> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();
        for (y, line) in s.trim().split('\n').enumerate() {
            let mut number_start: Option<Coord> = None;
            let mut number_value = String::new();
            for (x, char) in line.chars().enumerate() {
                if char.is_ascii_digit() {
                    if number_start.is_none() {
                        number_start = Some(Coord { x, y });
                    }
                    number_value.push(char);
                } else {
                    if char != '.' {
                        symbols.push(Coord { x, y });
                    }
                    if let Some(start) = number_start {
                        numbers.push(Number {
                            start,
                            end: Coord {
                                x: x.saturating_sub(1),
                                y,
                            },
                            value: number_value.parse()?,
                        });
                    }
                    number_start = None;
                    number_value = String::new();
                }
            }
            if let Some(start) = number_start {
                numbers.push(Number {
                    start,
                    end: Coord {
                        x: line.len() - 1,
                        y,
                    },
                    value: number_value.parse()?,
                });
            }
            number_start = None;
            number_value = String::new();
        }
        Ok(Self { symbols, numbers })
    }
}

fn part1(input: &str) -> Result<u32> {
    let engine = input.parse::<Engine>()?;
    Ok(engine
        .numbers
        .into_iter()
        .filter(|n| engine.symbols.iter().any(|s| n.adjacent_to(s)))
        .map(|n| n.value)
        .sum())
}

fn part2(input: &str) -> Result<u32> {
    let engine = input.parse::<Engine>()?;
    Ok(engine
        .symbols
        .into_iter()
        .filter_map(|s| {
            let adjacent_numbers = engine
                .numbers
                .iter()
                .filter(|n| n.adjacent_to(&s))
                .map(|n| n.value)
                .collect::<Vec<u32>>();
            if adjacent_numbers.len() == 2 {
                return Some(adjacent_numbers[0] * adjacent_numbers[1]);
            }
            None
        })
        .sum())
}

pub fn solve() -> Result<()> {
    println!("\nDay 03");
    instrument!(part1(INPUT)?, part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");

    #[test]
    fn test_adjacent_to() {
        assert!(Number {
            start: Coord { x: 5, y: 9 },
            end: Coord { x: 8, y: 9 },
            value: 598
        }
        .adjacent_to(&Coord { x: 4, y: 8 }),);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 467835);
    }
}
