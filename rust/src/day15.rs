use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day15.txt");

pub fn run() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers, 2020));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &'static str) -> Vec<u64> {
    input
        .trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(numbers: &[u64], until: usize) -> u64 {
    let mut last_spoken = *numbers.last().unwrap();
    let mut spoken_numbers: HashMap<u64, usize> = numbers
        .iter()
        .take(numbers.len() - 1)
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect();

    for idx in (numbers.len())..until {
        let current = match spoken_numbers.get(&last_spoken) {
            Some(prev_idx) => (idx - prev_idx) as u64,
            _ => 0,
        };

        spoken_numbers.insert(last_spoken, idx);
        last_spoken = current;
    }

    last_spoken
}

fn part2(numbers: &[u64]) -> u64 {
    part1(numbers, 30000000)
}
