use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/day05.txt");

type SeatID = u32;

pub fn run() {
    let seat_ids = transform_input(INPUT);

    println!("Part 1: {}", part1(&seat_ids));
    println!("Part 2: {}", part2(&seat_ids));
}

/// Convert each line in the input into a seat ID. We replace each letter with its
/// corresponding bit, and from that we calculate the seat ID.
///
/// Then sort it for part 2!
fn transform_input(input: &str) -> Vec<SeatID> {
    let mut seat_ids: Vec<SeatID> = input
        .trim_end()
        .split("\n")
        .map(|x| {
            // Any way to do this in place? Allocates a new string for each replace...
            let bin = x
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");

            let row = u32::from_str_radix(&bin[..7], 2).unwrap();
            let col = u32::from_str_radix(&bin[7..], 2).unwrap();

            row * 8 + col
        })
        .collect();

    seat_ids.sort();
    seat_ids
}

fn part1(seat_ids: &Vec<SeatID>) -> SeatID {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &Vec<SeatID>) -> SeatID {
    for (id, next_id) in seat_ids.iter().tuple_windows() {
        if id + 2 == *next_id {
            return id + 1;
        }
    }

    panic!("No seat ID found.");
}
