use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/day23.txt");

type Cup = usize;

// We solve this with a "cons vec/map"--represented as a vector where
// each index contains the value of the next index in the list.
//
// We can do this because the cups are all on the interval [1, n].

pub fn run() {
    let cups = transform_input(INPUT);

    println!("Part 1: {}", part1(&cups));
    println!("Part 2: {}", part2(&cups));
}

fn transform_input(input: &'static str) -> Vec<Cup> {
    input
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
}

fn part1(cups: &[Cup]) -> String {
    let mut cons_vec = create_cons_vec(cups);
    play_cups_game(&mut cons_vec, *cups.first().unwrap(), 100);
    get_cups_after_1(&cons_vec)
}

fn create_cons_vec(cups: &[Cup]) -> Vec<Cup> {
    let mut cons_vec: Vec<Cup> = Vec::new();
    cons_vec.resize(cups.len() + 1, 0);

    for idx in 0..cups.len() {
        cons_vec[cups[idx] as usize] = cups[(idx + 1) % cups.len()];
    }

    cons_vec
}

fn play_cups_game(cons_vec: &mut [Cup], first_cup: Cup, moves: usize) {
    let mut cur_cup = first_cup;

    for _ in 0..moves {
        let pickup_1 = cons_vec[cur_cup];
        let pickup_2 = cons_vec[pickup_1];
        let pickup_3 = cons_vec[pickup_2];

        // Calculate destination cup.
        let mut dest_cup = cur_cup - 1;
        while vec![pickup_1, pickup_2, pickup_3].contains(&dest_cup) || dest_cup < 1 {
            if dest_cup <= 1 {
                dest_cup = cons_vec.len() - 1;
            } else {
                dest_cup -= 1;
            }
        }

        let after_pickup = cons_vec[pickup_3];
        let after_dest = cons_vec[dest_cup];
        cons_vec[cur_cup] = after_pickup;
        cons_vec[dest_cup] = pickup_1;
        cons_vec[pickup_3] = after_dest;

        cur_cup = after_pickup;
    }
}

fn get_cups_after_1(cons_vec: &[Cup]) -> String {
    let mut cups: Vec<Cup> = Vec::new();
    cups.reserve(cons_vec.len() - 1);

    let mut next = cons_vec[1];
    while next != 1 {
        cups.push(next);
        next = cons_vec[next];
    }

    cups.iter().map(|x| x.to_string()).join("")
}

fn part2(cups: &[Cup]) -> usize {
    let mut cups = cups.to_owned();

    // Create the new million-element cups list.
    let max_cup = *cups.iter().max().unwrap();
    for cup in (max_cup + 1)..1_000_001 {
        cups.push(cup);
    }

    let mut cons_vec = create_cons_vec(&cups);
    play_cups_game(&mut cons_vec, *cups.first().unwrap(), 10_000_000);

    let first_cup = cons_vec[1];
    let second_cup = cons_vec[first_cup as usize];
    first_cup * second_cup
}
