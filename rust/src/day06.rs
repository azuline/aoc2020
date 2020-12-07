use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/day06.txt");

type AnswerGroup<'a> = Vec<&'a str>;

pub fn run() {
    let groups = transform_input(INPUT);

    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
}

fn transform_input(input: &str) -> Vec<AnswerGroup> {
    input
        .trim_end()
        .split("\n\n")
        .map(|x| x.split('\n').collect())
        .collect()
}

fn part1(groups: &[AnswerGroup]) -> usize {
    groups
        .iter()
        .map(|answers| {
            answers
                .iter()
                .flat_map(|x| x.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(groups: &[AnswerGroup]) -> usize {
    groups
        .iter()
        .map(|answers| {
            answers
                .iter()
                .map(|x| x.chars().collect::<HashSet<_>>())
                .fold_first(|acc, x| acc.intersection(&x).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}
