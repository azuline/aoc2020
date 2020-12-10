use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

// TODO: Figure out a way without cloning all these strings? And figure out all the weird
// ownership/borrowing shit with slices and strings and crap.

static INPUT: &str = include_str!("../../inputs/day07.txt");

lazy_static! {
    static ref BAG_AMOUNT_REGEX: Regex = Regex::new(r"(\d+) (.+?) bag").unwrap();
}

type BagMap<'a> = HashMap<String, Vec<String>>;
type BagMapWithCount<'a> = HashMap<String, Vec<(String, u32)>>;

pub fn run() {
    // One mapping each bag to the bags it contains.
    // The other mapping each bag to the bags that contain it.
    let (from_containing, to_containing) = transform_input(INPUT);

    println!("Part 1: {}", part1(&from_containing));
    println!("Part 2: {}", part2(&to_containing));
}

fn transform_input(input: &str) -> (BagMap, BagMapWithCount) {
    let mut from_containing: BagMap = HashMap::new();
    let mut to_containing: BagMapWithCount = HashMap::new();

    for rule in input.lines() {
        let (from_bag, to_bags) = rule.splitn(2, " bags contain ").next_tuple().unwrap();

        for cap in BAG_AMOUNT_REGEX.captures_iter(to_bags) {
            let num_bags: u32 = cap[1].parse().unwrap();

            to_containing
                .entry(from_bag.to_owned())
                .or_insert_with(Vec::new)
                .push((cap[2].to_owned(), num_bags));
            from_containing
                .entry(cap[2].to_owned())
                .or_insert_with(Vec::new)
                .push(from_bag.to_owned());
        }
    }

    (from_containing, to_containing)
}

fn part1(to_containing: &BagMap) -> usize {
    let mut seen: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    seen.push_back("shiny gold");

    while let Some(bag) = seen.pop_front() {
        if let Some(bags) = to_containing.get(bag) {
            for containing_bag in bags.iter() {
                if !visited.contains(&containing_bag[..]) {
                    seen.push_back(&containing_bag);
                }
            }
        }

        visited.insert(bag);
    }

    // Subtract 1 for the initial shiny gold bag.
    visited.len() - 1
}

fn part2(from_containing: &BagMapWithCount) -> u32 {
    let mut seen: VecDeque<(&str, u32)> = VecDeque::new();
    let mut total_bags = 0;

    seen.push_back(("shiny gold", 1));

    while let Some((bag, num_bags)) = seen.pop_front() {
        total_bags += num_bags;

        if let Some(bags) = from_containing.get(bag) {
            for (contained_bag, num_contained_bags) in bags.iter() {
                seen.push_back((contained_bag, num_bags * num_contained_bags));
            }
        }
    }

    // Subtract 1 for the initial shiny gold bag.
    total_bags - 1
}
