use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day10.txt");

pub fn run() {
    let numbers = transform_input(INPUT);

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn transform_input(input: &'static str) -> Vec<u64> {
    let mut numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    numbers.sort_unstable();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);

    numbers
}

fn part1(numbers: &[u64]) -> usize {
    let diffs: Vec<u64> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

    let off_ones = diffs.iter().filter(|x| **x == 1).count();
    let off_threes = diffs.iter().filter(|x| **x == 3).count();

    off_ones * off_threes
}

fn part2(numbers: &[u64]) -> u64 {
    fn calc_arrangements(numbers: &[u64], idx: usize, memo_table: &mut HashMap<usize, u64>) -> u64 {
        // Base case.
        if idx == numbers.len() - 2 {
            return 1;
        }

        if let Some(num_paths) = memo_table.get(&idx) {
            return *num_paths;
        };

        // Recursive case.

        let mut sum = 0;
        let cur = numbers[idx];

        if let Some(true) = numbers.get(idx + 3).map(|&x| x <= cur + 3) {
            sum += calc_arrangements(numbers, idx + 3, memo_table);
        }
        if let Some(true) = numbers.get(idx + 2).map(|&x| x <= cur + 2) {
            sum += calc_arrangements(numbers, idx + 2, memo_table);
        }

        sum += calc_arrangements(numbers, idx + 1, memo_table);
        memo_table.insert(idx, sum);
        sum
    };

    calc_arrangements(numbers, 0, &mut HashMap::new())
}
