pub mod day1;
pub mod day2;
pub mod days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Advent of code 2022");

    match args.get(1) {
        Some(day_input) => days::parse_advent_day(day_input),
        None => println!("Don't know which day do show."),
    }
}
