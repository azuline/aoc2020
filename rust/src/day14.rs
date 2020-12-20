use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day14.txt");

lazy_static! {
    static ref U64_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

type Memory = HashMap<u64, u64>;
type Mask = String;
type Assignment = (u64, u64);
type MaskAndAddrs = (Mask, Vec<Assignment>);

pub fn run() {
    let masks_and_addrs = transform_input(INPUT);

    println!("Part 1: {}", part1(&masks_and_addrs));
    println!("Part 2: {}", part2(&masks_and_addrs));
}

fn transform_input(input: &'static str) -> Vec<MaskAndAddrs> {
    let mut masks_and_addrs: Vec<MaskAndAddrs> = Vec::new();
    let mut cur_mask: Option<Mask> = None;
    let mut cur_assignments: Vec<Assignment> = Vec::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            if let Some(mask) = cur_mask {
                masks_and_addrs.push((mask, cur_assignments.clone()));
                cur_assignments.clear();
            }

            cur_mask = Some(String::from(line.split_at(7).1));
        } else {
            let m: Vec<u64> = U64_REGEX
                .find_iter(line)
                .map(|x| x.as_str().parse().unwrap())
                .collect();

            cur_assignments.push((m[0], m[1]));
        }
    }

    masks_and_addrs.push((cur_mask.unwrap(), cur_assignments));

    masks_and_addrs
}

fn apply_mask(mask: &str, value: u64) -> u64 {
    let value_bits: Vec<char> = format!("{:036b}", value).chars().collect();
    let masked_bits = mask
        .chars()
        .enumerate()
        .map(|(idx, c)| match c {
            'X' => value_bits[idx],
            c => c,
        })
        .collect::<String>();

    u64::from_str_radix(&masked_bits, 2).unwrap()
}

fn part1(masks_and_addrs: &[MaskAndAddrs]) -> u64 {
    let mut memory: Memory = HashMap::new();

    for (mask, assignments) in masks_and_addrs.iter() {
        for (location, value) in assignments.iter() {
            memory.insert(*location, apply_mask(&mask, *value));
        }
    }

    memory.values().sum()
}

fn part2(masks_and_addrs: &[MaskAndAddrs]) -> u64 {
    let mut memory: Memory = HashMap::new();

    for (mask, assignments) in masks_and_addrs.iter() {
        for (base_location, value) in assignments.iter() {
            let locations = generate_nondeterministic_locations(*base_location, &mask);
            for location in locations.iter() {
                memory.insert(*location, *value);
            }
        }
    }

    memory.values().sum()
}

fn generate_nondeterministic_locations(location: u64, mask: &str) -> Vec<u64> {
    let location_bits: Vec<char> = format!("{:036b}", location).chars().collect();

    let locations: Vec<String> = mask.chars().enumerate().fold(
        vec![String::from("")],
        |locs: Vec<String>, (idx, c)| {
            let next_chars: Vec<char> = match c {
                '0' => vec![location_bits[idx]],
                '1' => vec!['1'],
                'X' => vec!['0', '1'],
                _ => panic!("I WANT TO SLEEP."),
            };

            let mut new_locs = Vec::new();

            for s in locs.into_iter() {
                // TO be quite frank, this is an atrocity, but it's been an hour and I
                // want to sleep.
                for c in next_chars.iter() {
                    let mut new_str = s.clone();
                    new_str.push(*c);
                    new_locs.push(new_str);
                }
            }

            new_locs
        },
    );

    locations
        .iter()
        .map(|x| u64::from_str_radix(x, 2).unwrap())
        .collect()
}
