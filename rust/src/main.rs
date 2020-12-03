use std::env;
mod day01;
mod day02;
mod day03;

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
        _ => {
            println!("Invalid day~");
            return;
        }
    };
}
