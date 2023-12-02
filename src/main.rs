pub mod day01;
pub mod instrument;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Advent of Code 2023");
    day01::solve()?;
    Ok(())
}
