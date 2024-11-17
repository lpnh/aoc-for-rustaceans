//! Advent of Code for Rustaceans
//!
//! Let ~~rust~~ elves do the work for you!
//!
//! 🎄 Happy Coding! 🎄

#![warn(clippy::pedantic, clippy::nursery)]

mod advent;
pub use advent::{Advent, Day, SantaPackage};

/// ✨ macro magic acting as a placeholder for your awesome solution ✨
#[macro_export]
macro_rules! good_luck {
    ($input:ident) => {{
        let _ = &$input;
        Ok("".to_string())
    }};
}

/// ✨ macro magic to generate your `SantaPackage` ✨
#[macro_export]
macro_rules! elf_magic {
    () => {
        SantaPackage {
            day: CURRENT_DAY,
            puzzle_input: PUZZLE_INPUT.to_string(),
            solution_part_1: solve_part_1,
            solution_part_2: solve_part_2,
        }
    };
}

/// ✨ macro magic to test your solution for Part One ✨
#[macro_export]
macro_rules! test_part_one {
    () => {
        let input_example = _fmt(EXAMPLE_1);
        let solution = solve_part_1(&input_example).unwrap();
        assert_eq!(solution, EXPECTED_ANSWER_1);
    };
}

/// ✨ macro magic to test your solution for Part Two ✨
#[macro_export]
macro_rules! test_part_two {
    () => {
        let input_example = _fmt(EXAMPLE_2);
        let solution = solve_part_2(&input_example).unwrap();
        assert_eq!(solution, EXPECTED_ANSWER_2);
    };
}

/// A custom function to format multiline strings
pub fn _fmt(raw_str: &str) -> String {
    let lines: Vec<&str> = raw_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let min_indent = lines
        .iter()
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    lines
        .into_iter()
        .map(|line| line.get(min_indent..).unwrap_or(line))
        .collect::<Vec<&str>>()
        .join("\n")
}
