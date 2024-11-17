use anyhow::{Error, Result};
use std::fmt::{self, Display, Formatter};

/// The Days of Advent
#[derive(Debug, PartialEq, Eq)]
pub enum Day {
    /// 24 days until Christmas 🎄
    Day01,
    /// 23 days until Christmas 🎄
    Day02,
    /// 22 days until Christmas 🎄
    Day03,
    /// 21 days until Christmas 🎄
    Day04,
    /// 20 days until Christmas 🎄
    Day05,
    /// 19 days until Christmas 🎄
    Day06,
    /// 18 days until Christmas 🎄
    Day07,
    /// 17 days until Christmas 🎄
    Day08,
    /// 16 days until Christmas 🎄
    Day09,
    /// 15 days until Christmas 🎄
    Day10,
    /// 14 days until Christmas 🎄
    Day11,
    /// 13 days until Christmas 🎄
    Day12,
    /// 12 days until Christmas 🎄
    Day13,
    /// 11 days until Christmas 🎄
    Day14,
    /// 10 days until Christmas 🎄
    Day15,
    /// 9 days until Christmas 🎄
    Day16,
    /// 8 days until Christmas 🎄
    Day17,
    /// 7 days until Christmas 🎄
    Day18,
    /// 6 days until Christmas 🎄
    Day19,
    /// 5 days until Christmas 🎄
    Day20,
    /// 4 days until Christmas 🎄
    Day21,
    /// 3 days until Christmas 🎄
    Day22,
    /// 2 days until Christmas 🎄
    Day23,
    /// 1 day until Christmas 🎄
    Day24,
    /// 🌟 Merry Christmas! 🌟
    Day25,
}

/// Your `SantaPackage` 🎁
pub struct SantaPackage {
    pub day: Day,
    pub puzzle_input: String,
    pub solution_part_1: fn(&str) -> Result<String, Error>,
    pub solution_part_2: fn(&str) -> Result<String, Error>,
}

/// Your Advent Calendar 📆
pub struct Advent {
    pub day: Day,
    pub answer_part_1: String,
    pub answer_part_2: String,
}

impl Advent {
    /// Happy Holidays!
    ///
    /// 🦌🦌🦌🦌🦌🦌🦌🦌🦌 🎅
    pub fn ho_ho_ho(package: SantaPackage) -> Result<(), Error> {
        let advent = Self::get_package(package)?;

        println!("{advent}");

        Ok(())
    }

    fn get_package(package: SantaPackage) -> Result<Self, Error> {
        let advent = Self {
            day: package.day,
            answer_part_1: (package.solution_part_1)(&package.puzzle_input)?,
            answer_part_2: (package.solution_part_2)(&package.puzzle_input)?,
        };

        Ok(advent)
    }
}

impl Display for Advent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let star_1 = match self.answer_part_1.as_str() {
            "" => "",
            _ => "⭐",
        };
        let star_2 = match self.answer_part_2.as_str() {
            "" => "",
            _ => "⭐",
        };
        write!(
            f,
            "\n{:?} {}{}\n-----\nPart One: {}\nPart Two: {}\n",
            self.day, star_1, star_2, self.answer_part_1, self.answer_part_2
        )
    }
}
