use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("../../inputs/day02.txt");

lazy_static! {
    static ref POLICY_REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
}

#[derive(Debug)]
struct Policy<'a> {
    low: usize,
    high: usize,
    char: char,
    password: &'a str,
}

pub fn run() {
    let policies = transform_input(INPUT);

    println!("{:#?}", policies);
    println!("Part 1: {}", part1(&policies));
    println!("Part 2: {}", part2(&policies));
}

fn transform_input(input: &str) -> Vec<Policy> {
    input
        .split("\n")
        .filter_map(|x| {
            let caps = match POLICY_REGEX.captures(x) {
                Some(caps) => caps,
                None => return None,
            };

            Some(Policy {
                low: caps.get(1).unwrap().as_str().parse().unwrap(),
                high: caps.get(2).unwrap().as_str().parse().unwrap(),
                char: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                password: caps.get(4).unwrap().as_str(),
            })
        })
        .collect()
}

fn part1(policies: &Vec<Policy>) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let char_count = policy
                .password
                .chars()
                .filter(|&c| c == policy.char)
                .count();
            char_count >= policy.low && char_count <= policy.high
        })
        .count()
}

fn part2(policies: &Vec<Policy>) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let bytes = policy.password.as_bytes();

            (policy.char == *bytes.get(policy.low - 1).unwrap() as char)
                ^ (policy.char == *bytes.get(policy.high - 1).unwrap() as char)
        })
        .count()
}
