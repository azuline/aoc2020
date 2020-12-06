use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/day06.txt");

type AnswerGroups<'a> = Vec<Vec<&'a str>>;

pub fn run() {
    let groups = transform_input(INPUT);

    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
}

fn transform_input(input: &str) -> AnswerGroups {
    input
        .trim_end()
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect()
}

fn part1(groups: &AnswerGroups) -> usize {
    groups
        .iter()
        .map(|answers| {
            let mut seen: HashSet<char> = HashSet::new();
            for answer in answers.iter() {
                for c in answer.chars() {
                    seen.insert(c);
                }
            }
            seen.len()
        })
        .sum()
}

fn part2(answers: &AnswerGroups) -> usize {
    answers
        .iter()
        .map(|x| {
            let (head, tail) = x.split_first().unwrap();
            let initial: Vec<char> = head.chars().collect();
            tail.iter()
                .fold(initial, |acc, answer| {
                    answer.chars().filter(|c| acc.contains(c)).collect()
                })
                .len()
        })
        .sum()
}
