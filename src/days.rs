use crate::day1::main as day1;
use crate::day2::main as day2;
use std::fs::read_to_string;

pub enum Days {
    Day1,
    Day2,
}

impl Days {
    pub fn get_input(&self) -> String {
        match self {
            Days::Day1 => open_input_file("day1"),
            Days::Day2 => open_input_file("day2"),
        }
    }
}

pub fn run_day(number: usize) {
    let all_days = [day1, day2];

    match all_days.get(number - 1) {
        Some(day) => day(),
        None => get_error_msg(number),
    }
}

pub fn parse_advent_day(input: &String) {
    let adv_day = input.parse::<usize>();

    match adv_day {
        Ok(adv_day) => run_day(adv_day),
        Err(_) => println!("Could not parse provided advent day."),
    }
}

pub fn open_input_file(day: &str) -> String {
    let file_path = format!("./inputs/{}", day);

    match read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => panic!("Panicked lolz /n{err}"),
    }
}

fn get_error_msg(number: usize) {
    let allowed_range = 1..24;

    if allowed_range.contains(&number) {
        println!("Day not implemented yet.")
    } else {
        println!("Provide a number for the day, from 1 to 24.")
    }
}
