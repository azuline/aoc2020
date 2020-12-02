use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("../../inputs/day02.txt");

lazy_static! {
    static ref POLICY_REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
}

type Policy<'a> = (usize, usize, char, &'a str);

pub fn run() {
    let policies = transform_input(INPUT);

    println!("Part 1: {}", part1(&policies));
    println!("Part 2: {}", part2(&policies));
}

fn transform_input(input: &str) -> Vec<Policy> {
    input
        .lines()
        .filter_map(|x| {
            let caps = POLICY_REGEX.captures(x)?;

            Some((
                caps.get(1)?.as_str().parse().unwrap(),
                caps.get(2)?.as_str().parse().unwrap(),
                caps.get(3)?.as_str().chars().next()?,
                caps.get(4)?.as_str(),
            ))
        })
        .collect()
}

fn part1(policies: &Vec<Policy>) -> usize {
    policies
        .iter()
        .filter(|&(low, high, pol_char, pass)| {
            let char_count = pass.chars().filter(|c| c == pol_char).count();
            &char_count >= low && &char_count <= high
        })
        .count()
}

fn part2(policies: &Vec<Policy>) -> usize {
    policies
        .iter()
        .filter(|&(low, high, pol_char, pass)| {
            let mut chars = pass.chars();
            let lchar = chars.nth(low - 1).unwrap();
            let hchar = chars.nth(high - low - 1).unwrap();

            (pol_char == &lchar) ^ (pol_char == &hchar)
        })
        .count()
}
