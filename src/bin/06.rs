use aoc_for_rustaceans::*;

use anyhow::{Error, Result};

const CURRENT_DAY: Day = Day::Day06;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_06.txt");

fn solve_part_1(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

fn solve_part_2(input: &str) -> Result<String, Error> {
    good_luck!(input)
}

fn main() -> Result<(), Error> {
    Advent::ho_ho_ho(elf_magic!())?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_part_1() {
        const EXAMPLE_1: &str = "
            example input part 1 goes here
        ";

        const EXPECTED_ANSWER_1: &str = "";

        test_part_one!();
    }

    #[test]
    fn check_part_2() {
        const EXAMPLE_2: &str = "
            example input part 2 goes here
        ";

        const EXPECTED_ANSWER_2: &str = "";

        test_part_two!();
    }
}
