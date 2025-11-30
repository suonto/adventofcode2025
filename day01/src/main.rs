use day01::solve::*;
use std::env;

fn usage_and_exit() -> ! {
    eprintln!("Usage: day01 [a|b]  (a = part A, b = part B)");
    std::process::exit(2)
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| usage_and_exit());

    match arg.as_str() {
        "a" => println!("{}", solve_a(day01::examples::EXAMPLE_A)),
        "b" => println!("{}", solve_b(day01::examples::EXAMPLE_B)),
        _ => usage_and_exit(),
    }
}
