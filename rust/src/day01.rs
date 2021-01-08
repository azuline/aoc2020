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
    find_two_sum(numbers, 2020).unwrap()
}

fn find_two_sum(numbers: &[i32], target: i32) -> Option<i32> {
    let mut complements: HashSet<i32> = HashSet::new();

    for x in numbers.iter() {
        if complements.contains(x) {
            return Some(x * (target - x));
        }

        complements.insert(target - x);
    }

    None
}

fn part2(numbers: &[i32]) -> i32 {
    for x in numbers.iter() {
        if let Some(yz) = find_two_sum(numbers, 2020 - x) {
            return x * yz;
        }
    }

    panic!("No triplet found.");
}
