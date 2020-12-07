use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/day05.txt");

type SeatID = u32;

pub fn run() {
    let mut seat_ids = transform_input(INPUT);

    println!("Part 1: {}", part1(&seat_ids));
    println!("Part 2: {}", part2(&mut seat_ids));
}

/// Convert each line in the input into a seat ID. We replace each letter with its
/// corresponding bit, and from that we calculate the seat ID.
///
fn transform_input(input: &str) -> Vec<SeatID> {
    input
        .trim_end()
        .split('\n')
        .map(|x| {
            let bin = x
                .replace(&['F', 'L'][..], "0")
                .replace(&['B', 'R'][..], "1");

            u32::from_str_radix(&bin, 2).unwrap()
        })
        .collect()
}

fn part1(seat_ids: &[SeatID]) -> SeatID {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &mut [SeatID]) -> SeatID {
    seat_ids.sort_unstable();

    for (id, next_id) in seat_ids.iter().tuple_windows() {
        if id + 2 == *next_id {
            return id + 1;
        }
    }

    panic!("(Part 2) No seat ID found.");
}
