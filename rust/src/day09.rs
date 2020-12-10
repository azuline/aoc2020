use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/day09.txt");

pub fn run() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &'static str) -> Vec<i64> {
    input
        .trim_end()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(numbers: &[i64]) -> i64 {
    *numbers
        .iter()
        .skip(25)
        .enumerate()
        .find(|&(idx, elem)| !in_25_prev_sums(*elem, &numbers[idx..idx + 25]))
        .unwrap()
        .1
}

fn in_25_prev_sums(number: i64, numbers: &[i64]) -> bool {
    let mut complements: HashSet<i64> = HashSet::new();

    for x in numbers {
        if complements.contains(x) {
            return true;
        }

        complements.insert(number - x);
    }

    false
}

fn part2(numbers: &[i64]) -> i64 {
    let target = part1(numbers);
    let contiguous_range = find_contiguous_range(numbers, target);

    let min = contiguous_range.iter().min().unwrap();
    let max = contiguous_range.iter().max().unwrap();

    min + max
}

fn find_contiguous_range(numbers: &[i64], target: i64) -> &[i64] {
    let mut current_sum = numbers[0];
    let mut bottom = 0;
    let mut top = 0;

    while top < numbers.len() {
        if current_sum == target {
            return &numbers[bottom..top];
        }

        if current_sum > target {
            current_sum -= numbers[bottom];
            bottom += 1;
        } else {
            top += 1;
            current_sum += numbers[top];
        }
    }

    panic!("No contiguous range found.");
}
