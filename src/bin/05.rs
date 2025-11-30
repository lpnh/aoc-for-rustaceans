use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<String> {
    let result = input
        .lines()
        .next()
        .context("missing first line")?
        .replace("input", "output");

    Ok(result)
}

#[cfg(feature = "part_2")]
fn solution_part_2(input: &str) -> Result<String> {
    let result = input
        .lines()
        .next()
        .context("missing first line")?
        .replace("input", "output");

    Ok(result)
}

fn main() -> Result<()> {
    #[cfg(feature = "part_1")]
    println!("Part One: {}", solution_part_1(PUZZLE_INPUT)?);

    #[cfg(feature = "part_2")]
    println!("Part Two: {}", solution_part_2(PUZZLE_INPUT)?);

    Ok(())
}

#[cfg(feature = "part_1")]
#[test]
fn test_part_1() -> Result<()> {
    const EXAMPLE_INPUT_1: &str = "\
Part One example input
";

    const EXAMPLE_OUTPUT_1: &str = "Part One example output";

    assert_eq!(solution_part_1(EXAMPLE_INPUT_1)?, EXAMPLE_OUTPUT_1);

    Ok(())
}

#[cfg(feature = "part_2")]
#[test]
fn test_part_2() -> Result<()> {
    const EXAMPLE_INPUT_2: &str = "\
Part Two example input
";

    const EXAMPLE_OUTPUT_2: &str = "Part Two example output";

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
