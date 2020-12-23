use itertools::Itertools;
use std::collections::VecDeque;

static INPUT: &str = include_str!("../../inputs/day23.txt");

type Cups = VecDeque<u64>;

pub fn run() {
    let cups = transform_input(INPUT);

    println!("Part 1: {}", part1(&cups));
    println!("Part 2: {}", part2(&cups));
}

fn transform_input(input: &'static str) -> Cups {
    input
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect()
}

fn part1(cups: &Cups) -> String {
    let mut cups = cups.clone();
    play_cups_game(&mut cups, 100);
    cups.iter().skip(1).map(|x| x.to_string()).join("")
}

fn play_cups_game(cups: &mut Cups, moves: usize) {
    // Mutable temporary storage array for picked up cups.
    let mut picked_up: [u64; 3] = [0, 0, 0];

    let max_cup = *cups.iter().max().unwrap();
    let min_cup = *cups.iter().min().unwrap();

    for i in 0..moves {
        if i % 100 == 0 {
            dbg!(format!("ON MOVE {}", i));
        }

        let cur_cup = cups[0];

        picked_up[0] = cups.remove(1).unwrap();
        picked_up[1] = cups.remove(1).unwrap();
        picked_up[2] = cups.remove(1).unwrap();

        let dest_idx = find_dest_cup_idx(&cups, &picked_up, cur_cup, min_cup, max_cup);

        cups.insert(dest_idx + 1, picked_up[0]);
        cups.insert(dest_idx + 2, picked_up[1]);
        cups.insert(dest_idx + 3, picked_up[2]);

        cups.rotate_left(1);
    }

    while cups[0] != 1 {
        cups.rotate_left(1);
    }
}

fn find_dest_cup_idx(
    cups: &Cups,
    picked_up: &[u64],
    cur_cup: u64,
    min_cup: u64,
    max_cup: u64,
) -> usize {
    let mut dest_cup = cur_cup - 1;

    while picked_up.contains(&dest_cup) || dest_cup < min_cup {
        if dest_cup < min_cup {
            dest_cup = max_cup;
        } else {
            dest_cup -= 1;
        }
    }

    cups.iter().position(|&x| x == dest_cup).unwrap()
}

fn part2(cups: &Cups) -> u64 {
    let mut cups = cups.clone();

    // Create the new million-element cups list.
    let max_cup = *cups.iter().max().unwrap();
    for cup in (max_cup + 1)..1_000_001 {
        cups.push_back(cup);
    }

    play_cups_game(&mut cups, 10_000_000);

    cups[1] * cups[2]
}
