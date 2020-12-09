use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/day09.txt");

pub fn run() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &'static str) -> Vec<u64> {
    input
        .trim_end()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(numbers: &[u64]) -> u64 {
    *numbers
        .iter()
        .skip(25)
        .enumerate()
        .find(|&(idx, elem)| !in_25_prev_sums(*elem, &numbers[idx..idx + 25]))
        .unwrap()
        .1
}

fn in_25_prev_sums(number: u64, numbers: &[u64]) -> bool {
    numbers
        .iter()
        .combinations(2)
        .any(|c| number == c[0] + c[1])
}

fn part2(numbers: &[u64]) -> u64 {
    let target = part1(numbers);
    let contiguous_range = find_contiguous_range(numbers, target);

    let min = contiguous_range.iter().min().unwrap();
    let max = contiguous_range.iter().max().unwrap();

    min + max
}

fn find_contiguous_range(numbers: &[u64], target: u64) -> &[u64] {
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
