use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = include_str!("../_inputs/day01.txt");

fn main() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &str) -> Vec<u32> {
    input.split("\n").filter_map(|x| x.parse().ok()).collect()
}

fn part1(numbers: &Vec<u32>) -> u32 {
    let complements: HashSet<u32> = numbers.iter().map(|x| 2020 - x).collect();
    let found = numbers.iter().find(|x| complements.contains(x)).unwrap();
    found * (2020 - found)
}

fn part2(numbers: &Vec<u32>) -> u32 {
    let complements: HashSet<u32> = numbers.iter().map(|x| 2020 - x).collect();

    let (x, y) = numbers
        .into_iter()
        .tuple_combinations()
        .find(|&tuple| {
            let (x, y) = tuple;
            complements.contains(&(x + y))
        })
        .unwrap();

    x * y * (2020 - x - y)
}
