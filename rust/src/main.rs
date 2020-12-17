#![feature(iterator_fold_self)]

use std::env;

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
        Ok(8) => day08::run(),
        Ok(9) => day09::run(),
        Ok(10) => day10::run(),
        Ok(11) => day11::run(),
        Ok(12) => day12::run(),
        Ok(13) => day13::run(),
        Ok(14) => day14::run(),
        Ok(15) => day15::run(),
        Ok(16) => day16::run(),
        _ => {
            println!("Invalid day~");
        }
    }
}
