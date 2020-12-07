use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/day01.txt");

pub fn run() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &str) -> Vec<i32> {
    input.lines().filter_map(|x| x.parse().ok()).collect()
}

fn part1(numbers: &[i32]) -> i32 {
    let mut complements: HashSet<i32> = HashSet::new();

    for x in numbers.iter() {
        if complements.contains(x) {
            return x * (2020 - x);
        }

        complements.insert(2020 - x);
    }

    panic!("No pair found.")
}

fn part2(numbers: &[i32]) -> i32 {
    let mut complements: HashSet<i32> = HashSet::new();

    for (x, y) in numbers.iter().tuple_combinations() {
        if complements.contains(&(x + y)) {
            return x * y * (2020 - x - y);
        }

        complements.insert(2020 - x - y);
    }

    panic!("No triplet found.");
}
