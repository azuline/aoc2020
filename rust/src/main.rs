#![feature(iterator_fold_self)]

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please invoke with a day~");
        return;
    }

    match args[1].parse() {
        Ok(1) => day01::run(),
        Ok(2) => day02::run(),
        Ok(3) => day03::run(),
        Ok(4) => day04::run(),
        Ok(5) => day05::run(),
        Ok(6) => day06::run(),
        Ok(7) => day07::run(),
        _ => {
            println!("Invalid day~");
        }
    }
}
