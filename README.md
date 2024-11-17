# Advent of Code Template

A Rust template for the [Advent of Code](https://adventofcode.com/).  
*Let ~~Rust~~ elves do the hard work for you!*

## Usage

### Solving

Write your solution for Part One and Part Two in the `solve_part_1` and
`solve_part_2` functions, respectively.

The function is expecting you to return `Result<String, anyhow::Error>`.
This means that you can return either `Ok(String)` or `Err(anyhow::Error)`.
The easiest way to achieve this is to simply wrap your solution with `Ok`:

```rust
fn answer_to_ultimate_question() -> Result<String, Error> {
    let result = "42".to_string();
    Ok(result)
}
```

and use a `?` to propagate any potential errors:

```rust
fn answer_to_ultimate_question() -> Result<i32, Error> {
    let result = "42".parse::<i32>()?;
    Ok(result)
}
```

Note: The code compiles right from the start. This means you can try to test
and solve the Part One without worrying about the Part Two.

### Testing

With the examples provided by the Advent of Code, you can run a test for your
solutions if you want to.

Inside the `tests` module, you can find the `check_part_1` and `check_part_2` functions.

Update `EXAMPLE_1` or `EXAMPLE_2` constant and the respective
`EXPECTED_ANSWER_1` or `EXPECTED_ANSWER_2` to match the example for each part.
Such as:

```rust
fn check_part_1() {
    const EXAMPLE_1: &str = "
        example input for the part 1
        note: you can use indented
        multiline strings just like this,
        or non-indented ones as well.
        do as you wish,
        the elves will handle both!
    ";

    const EXPECTED_ANSWER_1: &str = "42";

    test_part_one!();
}
```

To test your solution, you can run `cargo test`, with the `--bin` flag to
specify the day you want to test. For example, to test the Day 1:

```no_rust
cargo test --bin 01
```

You can also run `cargo test --bins` to test all days at once.

Note: Both are passing by default. You can use them to test only the Part One,
only the Part Two, or both.

### Getting the Answer

You first need to insert the puzzle input inside the `day_XX.txt` file. You can
find it inside the `puzzle_input` folder.

Then, use `cargo run` with the `--bin` flag to specify the day you want to run.
For example, to run the day 01:

```no_rust
cargo run --bin 01
```

Your answer will be printed in the terminal. Like this:

```no_rust

Day01
-----
Part One: 
Part Two: 

```

---

🎄 Happy coding! 🎄
