use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("../../inputs/day04.txt");
static ECL_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

type Passport<'a> = HashMap<&'a str, &'a str>;
type ValidatorResult = Result<bool, <i32 as FromStr>::Err>;
type Validator<'a> = (&'a str, Box<dyn Fn(&'a str) -> ValidatorResult>);

pub fn run() {
    let passports = transform_input(INPUT);

    let validators: Vec<Validator> = vec![
        ("byr", Box::new(|x| check_range(x, 1920, 2002))),
        ("iyr", Box::new(|x| check_range(x, 2010, 2020))),
        ("eyr", Box::new(|x| check_range(x, 2020, 2030))),
        (
            "hgt",
            Box::new(|x| {
                if x.ends_with("cm") {
                    check_range(x.get(0..x.len() - 2).unwrap(), 150, 193)
                } else if x.ends_with("in") {
                    check_range(x.get(0..x.len() - 2).unwrap(), 59, 76)
                } else {
                    Ok(false)
                }
            }),
        ),
        ("hcl", Box::new(|x| Ok(HCL_REGEX.is_match(x)))),
        ("ecl", Box::new(|x| Ok(ECL_COLORS.contains(&x)))),
        ("pid", Box::new(|x| Ok(PID_REGEX.is_match(x)))),
    ];

    println!("Part 1: {}", part1(&passports, &validators));
    println!("Part 2: {}", part2(&passports, &validators));
}

fn check_range(value: &str, lower: i32, upper: i32) -> ValidatorResult {
    let value: i32 = value.parse()?;
    Ok(value >= lower && value <= upper)
}

fn transform_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|x| x.splitn(2, ':').collect_tuple().unwrap())
                .collect()
        })
        .collect()
}

fn part1(passports: &[Passport], validators: &[Validator]) -> usize {
    passports
        .iter()
        .filter(|passport| validators.iter().all(|(key, _)| passport.contains_key(key)))
        .count()
}

fn part2<'a>(passports: &'a [Passport], validators: &[Validator<'a>]) -> usize {
    passports
        .iter()
        .filter(|&passport| {
            validators.iter().all(|(key, validator)| {
                passport.contains_key(key)
                    && Ok(true) == validator(&passport.get(key).unwrap())
            })
        })
        .count()
}
