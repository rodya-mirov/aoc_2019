use std::process;
use std::time::Instant;

use clap::{App, Arg};

mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("1.0")
        .arg(Arg::from_usage(
            "-n, --number=<DAY_NUMBER> 'e.g. 2b for the second half of day 2'",
        ))
        .get_matches();

    let number = matches.value_of("number").unwrap();

    let start = Instant::now();

    match number {
        "1a" => day1::a(),
        "1b" => day1::b(),

        "2a" => day2::a(),
        "2b" => day2::b(),

        "3a" => day3::a(),
        "3b" => day3::b(),

        "4a" => day4::a(),
        "4b" => day4::b(),

        "5a" => day5::a(),
        "5b" => day5::b(),

        "6a" => day6::a(),
        "6b" => day6::b(),

        "7a" => day7::a(),
        "7b" => day7::b(),

        _ => {
            eprintln!("Unrecognized day combination: {}", number);
            process::exit(1);
        }
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;
    println!("Problem {} took {} ms", number, elapsed_ms);
}
