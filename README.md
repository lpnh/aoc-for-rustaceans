# AoC for Rustaceans

A silly [Rust](https://rust-lang.org/) boilerplate template for the [Advent of
Code](https://adventofcode.com/)

## Overview

### Days

In this template, each **day** of the Advent of Code has its own dedicated
file:

```txt
.
├── puzzle_input
│   ├── day_01.txt
│   ├── day_02.txt
│   ├── day_03.txt
│   ├── day_04.txt
│   ├── day_05.txt
│   ├── day_06.txt
│   ├── day_07.txt
│   ├── day_08.txt
│   ├── day_09.txt
│   ├── day_10.txt
│   ├── day_11.txt
│   └── day_12.txt
└── src
    └── bin
        ├── 01.rs
        ├── 02.rs
        ├── 03.rs
        ├── 04.rs
        ├── 05.rs
        ├── 06.rs
        ├── 07.rs
        ├── 08.rs
        ├── 09.rs
        ├── 10.rs
        ├── 11.rs
        └── 12.rs
```

As you can notice, each day is treated as an isolated binary.

### Parts

This template allows you to test and solve the two parts of the puzzle
independently. Using the functions `solution_part_1` and `test_part_1` for the
first part, and `solution_part_2` and `test_part_2` for the second one.

To be able to make the Part One and Part Two truly independent, this template
leverages Rust's feature flags system.
They can be recognized by their unique syntax: `#[cfg(feature = "part_1")]`.
Both `part_1` and `part_2` are enabled (will be compiled and executed) by
default.

## Usage

### Input Setup

Update the `EXAMPLE_INPUT_*` and `EXAMPLE_OUTPUT_*` constants inside the
`test_part_*` functions with the values provided by the puzzle description
*example*.
Update the `puzzle_input/day_*.txt` file with your *puzzle input*.

### Solution

Implement your solution using the `solution_part_*` functions.
The return type of these functions is `Result<String>`, which means you can
wrap your result with `Ok()` and use the `?` operator to propagate any
potential errors.

### Cargo

Use `cargo test` to test your solution, running it against the *example input*.
And `cargo run` to get the *puzzle answer*, running the solution against the
*puzzle input*.

This template includes some convenient cargo aliases defined in
`.cargo/config.toml`. These provide shorthand aliases: `t`, `t1`, and `t2`, for
testing and `r`, `r1`, `r2` for running.
Below you can see how they could be used for the Day 1, as an example.

```sh
# Test only part 1
cargo t1 01

# Test only part 2
cargo t2 01

# Test both parts
cargo t 01

```

```sh
# Run only part 1
cargo r1 01

# Run only part 2
cargo r2 01

# Run both parts
cargo r 01
```

You can also use the standard cargo commands, for example:

```sh
# Test only part 1
cargo test --bin 01 --no-default-features -F part_1

# Test only part 2
cargo test --bin 01 --no-default-features -F part_2

# Test both parts
cargo run --bin 01
```

```sh
# Run only part 1
cargo run --bin 01 --no-default-features -F part_1

# Run only part 2
cargo run --bin 01 --no-default-features -F part_2

# Run both parts
cargo run --bin 01
```

> [!NOTE]
> The verbosity of the commands is a result of the somewhat unconventional
> workspace structure of this template.

---

🎄 *Happy coding!* 🎄
