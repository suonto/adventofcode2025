# Advent of Code Rust Workspace

This repository uses a Cargo workspace where each "day" is a separate crate.

See: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

There is a crate for each day, and one example shared library for a Jagger that moves.

## Intended Workflow

1. Copy paste the example to day01 part A.
2. Implement day01 solve_a.
3. Add a test so that `solve_a(EXAMPLE_A)` returns expected.
4. Copy paste your real input to some example and run it to report results but never commit your input.

## Run DayNN Part A or B

Run part A or B by passing `a` or `b` as the first argument to main:

```bash
cargo run -p day01 -- a   # run day 01 part A
cargo run -p day01 -- b   # run day 02 part B
```

## Run Tests

Run tests for a single package by package name:

```bash
cargo test -p day01 -q
```
