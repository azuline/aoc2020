static INPUT: &str = include_str!("../../inputs/day18.txt");

#[derive(Debug)]
pub enum Math {
    Expr(Box<Math>, char, Box<Math>),
    Int(u64),
}

pub fn run() {
    let data = transform_input(INPUT);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

pub fn transform_input(input: &'static str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(data: &[&str]) -> u64 {
    data.iter().map(|line| eval_math(&parse(line))).sum()
}

/// Ok... so a little grammar for a recursive descent parser.
///
/// So this is kind of crappy, but this parser is right-associative
/// while the math should be left-associative. I'm a bit tired so I'm
/// just gonna reverse the chars and flip the parens! Hahaha what a
/// terrible hack!
///
/// math = add | mult | expr
/// add = expr "+" math
/// mult = expr "*" math
/// expr = ")" math "(" | int

pub fn parse(line: &str) -> Math {
    let chars: Vec<char> = line.chars().filter(|&c| c != ' ').rev().collect();
    parse_math(&chars, 0).unwrap().0
}

fn parse_math(chars: &[char], idx: usize) -> Option<(Math, usize)> {
    let (left, idx) = parse_expr(chars, idx)?;

    // If we match operator, then this is add/mult. Otherwise, it's just
    // integer expr.
    if let Some((oper, idx)) = match_char(chars, idx, &['+', '*']) {
        let (right, idx) = parse_math(chars, idx)?;
        Some((Math::Expr(Box::new(left), oper, Box::new(right)), idx))
    } else {
        Some((left, idx))
    }
}

fn parse_expr(chars: &[char], idx: usize) -> Option<(Math, usize)> {
    if let Some((_, idx)) = match_char(chars, idx, &[')']) {
        let (expr, idx) = parse_math(chars, idx)?;
        let (_, idx) = match_char(chars, idx, &['('])?;

        return Some((expr, idx));
    };

    match_int(chars, idx)
}

fn match_char(chars: &[char], idx: usize, matching: &[char]) -> Option<(char, usize)> {
    match chars.get(idx) {
        Some(c) if matching.contains(c) => Some((*c, idx + 1)),
        _ => None,
    }
}

fn match_int(chars: &[char], idx: usize) -> Option<(Math, usize)> {
    let int = chars.get(idx)?.to_digit(10)?;
    Some((Math::Int(int as u64), idx + 1))
}

fn eval_math(math: &Math) -> u64 {
    match math {
        Math::Expr(left, '+', right) => eval_math(left) + eval_math(right),
        Math::Expr(left, '*', right) => eval_math(left) * eval_math(right),
        Math::Int(int) => *int,
        _ => panic!("Invalid math expression."),
    }
}

fn part2(data: &[&str]) -> u64 {
    data.iter()
        .map(|line| eval_math(&parse(&parenthesize_line(line))))
        .sum()
}

fn parenthesize_line(line: &str) -> String {
    let mut rval = String::from("((((");
    for c in line.chars() {
        match c {
            '(' => rval.push_str("(((("),
            ')' => rval.push_str("))))"),
            '+' => rval.push_str("))+(("),
            '*' => rval.push_str(")))*((("),
            c => rval.push(c),
        }
    }
    rval.push_str("))))");
    rval
}
