use std::process;
use std::time::Instant;

use clap::{App, Arg};

mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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
        "1a" => day01::a(),
        "1b" => day01::b(),

        "2a" => day02::a(),
        "2b" => day02::b(),

        "3a" => day03::a(),
        "3b" => day03::b(),

        "4a" => day04::a(),
        "4b" => day04::b(),

        "5a" => day05::a(),
        "5b" => day05::b(),

        "6a" => day06::a(),
        "6b" => day06::b(),

        "7a" => day07::a(),
        "7b" => day07::b(),

        "8a" => day08::a(),
        "8b" => day08::b(),

        "9a" => day09::a(),
        "9b" => day09::b(),

        "10a" => day10::a(),
        "10b" => day10::b(),

        "11a" => day11::a(),
        "11b" => day11::b(),

        "12a" => day12::a(),
        "12b" => day12::b(),

        "13a" => day13::a(),
        "13b" => day13::b(),

        "14a" => day14::a(),
        "14b" => day14::b(),

        "15a" => day15::a(),
        "15b" => day15::b(),

        "16a" => day16::a(),
        "16b" => day16::b(),

        _ => {
            eprintln!("Unrecognized day combination: {}", number);
            process::exit(1);
        }
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;
    println!("Problem {} took {} ms", number, elapsed_ms);
}
