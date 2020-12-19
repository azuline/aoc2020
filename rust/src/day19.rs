use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Debug, Clone)]
enum RHS {
    Terminal(char),
    Nonterminal(u32),
}
use RHS::*;

type Productions = HashMap<u32, Vec<Vec<RHS>>>;

pub fn run() {
    let (productions, messages) = transform_input(INPUT);

    println!("Part 1: {}", part1(&productions, &messages));
    println!("Part 2: {}", part2(&productions, &messages));
}

fn transform_input(input: &'static str) -> (Productions, Vec<&'static str>) {
    let (in_rules, in_messages) = input.split("\n\n").next_tuple().unwrap();

    let productions = in_rules
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.splitn(2, ": ").next_tuple().unwrap();
            let production = lhs.parse().unwrap();
            let rules = rhs
                .split(" | ")
                .map(|sub_rule| {
                    sub_rule
                        .split(' ')
                        .map(|x| {
                            if let Ok(int) = x.parse::<u32>() {
                                Nonterminal(int)
                            } else {
                                Terminal(x.chars().nth(1).unwrap())
                            }
                        })
                        .collect()
                })
                .collect();

            (production, rules)
        })
        .collect();

    let messages = in_messages.lines().collect();

    (productions, messages)
}

fn part1(productions: &Productions, messages: &[&'static str]) -> usize {
    messages
        .iter()
        .filter(|x| match_rule(productions, 0, x).iter().any(|o| o.is_empty()))
        .count()
}

fn match_rule(productions: &Productions, rule: u32, input: &'static str) -> Vec<&'static str> {
    productions
        .get(&rule)
        .unwrap()
        .iter()
        .flat_map(|sub_rule| {
            sub_rule.iter().fold(vec![input], |acc, rhs| {
                acc.iter()
                    .filter_map(|x| match rhs {
                        Terminal(c) if x.chars().next()? == *c => Some(vec![&x[1..]]),
                        Nonterminal(new_rule) => Some(match_rule(productions, *new_rule, x)),
                        _ => None,
                    })
                    .flatten()
                    .collect()
            })
        })
        .collect()
}

fn part2(productions: &Productions, messages: &[&'static str]) -> usize {
    part1(&modify_productions(productions), messages)
}

fn modify_productions(productions: &Productions) -> Productions {
    let mut productions = productions.clone();

    productions.insert(
        8,
        vec![vec![Nonterminal(42)], vec![Nonterminal(42), Nonterminal(8)]],
    );
    productions.insert(
        11,
        vec![
            vec![Nonterminal(42), Nonterminal(31)],
            vec![Nonterminal(42), Nonterminal(11), Nonterminal(31)],
        ],
    );

    productions
}
