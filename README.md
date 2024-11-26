# AoC for Rustaceans

A Rust boilerplate template for the [Advent of Code](https://adventofcode.com/)

Ensure Rust is installed. You can find the official setup guide
[here](https://www.rust-lang.org/learn/get-started)

## Puzzle Input

This template assumes two distinct input types provided by the *Advent of Code* puzzles:

- **Sample input**: Provided in the puzzle description to illustrate the
problem. Use this in test functions to validate your logic.
- **Personal puzzle input**: Your unique puzzle input, which is necessary to
get the actual puzzle answer. Place this input in the `puzzle_input/day_XX.txt`
file.

> [!TIP]
> A good approach to solving the puzzle with this template is to first *test*
> your solution using `cargo test` before attempting to *run* it. Testing uses
> the **sample input**, which provides a quick way to validate your algorithm’s
> basic functionality before running it on the larger, potentially more
> time-consuming, **personal input**.

## Writing Solutions

Implement your solutions in the `solve_part_1` and `solve_part_2` functions.

### Example for Part One

```rust
fn solve_part_1(input: &str) -> Result<String, Error> {
    // Replace the placeholder solution with your own

    Ok(solution)
}
```

> [!NOTE]
> The function's return type is `Result<String, anyhow::Error>`. This means
> your solution must return either `Ok(String)` or `Err(anyhow::Error)`. The
> easiest way to achieve this is to wrap your result with `Ok()` and use the
> `?` operator to propagate any potential errors.

## Running the Solution

### Solve a Specific Day

To solve the puzzle for a specific day, use the `--bin` flag with the day
number:

```bash
cargo run --bin 01
```

By default, the output looks like this:

```
Day 01
------
```

> [!NOTE]
> This template uses a custom structure where each day’s solution is treated as
> a separate binary. These binaries are placed in the src/bin/ directory,
> deviating from the default main.rs. This allows us to isolate each day’s
> logic and execute it independently using the --bin flag.

### Run Individual Parts

You can run individual parts using feature flags:

```bash
cargo run --bin 01 --features part_1
```

```bash
cargo run --bin 01 --features part_2
```

### Run Both Parts Together

There are two ways to run both parts:

1. Using the `--all-features` flag:

   ```bash
   cargo run --bin 01 --all-features
   ```

2. Using a space-separated or comma-separated list of features:

   ```bash
   cargo run --bin 01 --features "part_1 part_2"
   ```

   ```bash
   cargo run --bin 01 --features part_1,part_2
   ```

The default output when running both parts looks like this:

```
Day 01
------
Part One: <your puzzle answer>
Part Two: <your puzzle answer>
```

> [!NOTE]
> Feature flags in Rust enable conditional compilation of code. In this
> template, they are used to execute solutions for specific parts of the puzzle
> (`part_1` or `part_2`).

## Testing Your Code

Tests are included for both parts. Update the constants with the sample input
and its corresponding answer.

### Example for Part One Test

```rust
#[test]
fn sample_part_1() {
    const SAMPLE_INPUT_1: &str = "\
sample part 1 input
goes here
like this
";
    const SAMPLE_ANSWER_1: &str = "sample part 1 answer";

    assert_eq!(solve_part_1(SAMPLE_INPUT_1).unwrap(), SAMPLE_ANSWER_1);
}
```

### Run the Tests

You can run the tests for your solution using the `cargo test` command. Use the
`--bin` flag to specify the day and `--features` to include a specific part, or
`--all-features` to test both:

- To test Part One:

  ```bash
  cargo test --bin 01 --features part_1
  ```

- To test Part Two:

  ```bash
  cargo test --bin 01 --features part_2
  ```

- To test both parts:

  ```bash
  cargo test --bin 01 --all-features
  ```

---

🎄 *Happy coding!* 🎄
