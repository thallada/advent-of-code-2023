use std::cmp::min;

use anyhow::{anyhow, Result};
use phf::phf_map;

use crate::instrument::instrument;

const INPUT: &str = include_str!("input/input.txt");
static DIGITS: phf::Map<&[u8], u8> = phf_map! {
    b"zero" => 0,
    b"one" => 1,
    b"two" => 2,
    b"three" => 3,
    b"four" => 4,
    b"five" => 5,
    b"six" => 6,
    b"seven" => 7,
    b"eight" => 8,
    b"nine" => 9,
    b"0" => 0,
    b"1" => 1,
    b"2" => 2,
    b"3" => 3,
    b"4" => 4,
    b"5" => 5,
    b"6" => 6,
    b"7" => 7,
    b"8" => 8,
    b"9" => 9,
};

fn find_num(line: &[u8], reverse: bool) -> Option<u8> {
    let window_size = min(line.len(), 5);
    for i in 0..line.len() {
        for j in 1..min(window_size + 1, line.len() - i + 1) {
            if j == 2 {
                continue; // no two-byte digits
            }
            if !reverse {
                if let Some(digit) = DIGITS.get(&line[i..i + j]) {
                    return Some(*digit);
                }
            } else if let Some(digit) = DIGITS.get(&line[line.len() - i - j..line.len() - i]) {
                return Some(*digit);
            }
        }
    }
    None
}

fn part1(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.trim().split('\n') {
        let first = line
            .as_bytes()
            .iter()
            .find(|c| c.is_ascii_digit())
            .map(|c| c - b'0')
            .ok_or_else(|| anyhow!("Invalid input: no digits found in line {}", line))?;
        let last = line
            .as_bytes()
            .iter()
            .rev()
            .find(|c| c.is_ascii_digit())
            .map(|c| c - b'0')
            .ok_or_else(|| anyhow!("Invalid input: no digits found in line {}", line))?;
        sum += (first as u32 * 10) + last as u32;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.trim().split('\n') {
        let first = find_num(line.as_bytes(), false)
            .ok_or_else(|| anyhow!("Invalid input: no digits found in line {}", line))?;
        let last = find_num(line.as_bytes(), true)
            .ok_or_else(|| anyhow!("Invalid input: no digits found in line {}", line))?;
        sum += (first as u32 * 10) + last as u32;
    }
    Ok(sum)
}

pub fn solve() -> Result<()> {
    println!("Day 01");
    instrument!(part1(INPUT)?, part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("input/test1.txt");
    const TEST_INPUT2: &str = include_str!("input/test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1).unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1).unwrap(), 142);
        assert_eq!(part2(TEST_INPUT2).unwrap(), 281);
    }
}
