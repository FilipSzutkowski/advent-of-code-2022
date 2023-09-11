use crate::day1::main as day1;

pub fn run_day(number: usize) {
    let all_days = [day1];

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

fn get_error_msg(number: usize) {
    let allowed_range = 1..24;

    if allowed_range.contains(&number) {
        println!("Day not implemented yet.")
    } else {
        println!("Provide a number for the day, from 1 to 24.")
    }
}
