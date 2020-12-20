use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

static INPUT: &str = include_str!("../../inputs/day16.txt");

lazy_static! {
    static ref FIELD_REGEX: Regex =
        Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

type Fields = HashMap<String, Vec<Range<u64>>>;
type Ticket = Vec<u64>;

struct Data {
    fields: Fields,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

pub fn run() {
    let data = transform_input(INPUT);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn transform_input(input: &'static str) -> Data {
    let (in_fields, in_ticket, in_nearby) = input.split("\n\n").next_tuple().unwrap();

    let fields: Fields = in_fields
        .lines()
        .map(|line| {
            let m = FIELD_REGEX.captures(&line).unwrap();
            let name = m[1].to_owned();
            let ranges = vec![
                Range {
                    start: m[2].parse().unwrap(),
                    end: m[3].parse::<u64>().unwrap() + 1,
                },
                Range {
                    start: m[4].parse().unwrap(),
                    end: m[5].parse::<u64>().unwrap() + 1,
                },
            ];

            (name, ranges)
        })
        .collect();

    let your_ticket = in_ticket
        .strip_prefix("your ticket:\n")
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let nearby_tickets = in_nearby
        .strip_prefix("nearby tickets:\n")
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    Data {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

fn part1(data: &Data) -> u64 {
    data.nearby_tickets
        .iter()
        .flatten()
        .filter(|x| {
            !data
                .fields
                .values()
                .flatten()
                .any(|range| range.contains(&x))
        })
        .sum()
}

/// Going to go for the greedy-matching solution. If it doesn't work
/// for all inputs, so be it!
fn part2(data: &Data) -> u64 {
    let field_values = get_field_values(&data.nearby_tickets, &data.fields);

    let mut possible_placements: HashMap<&String, Vec<usize>> = data
        .fields
        .iter()
        .map(|(name, ranges)| {
            let placements = field_values
                .iter()
                .enumerate()
                .filter_map(|(field_idx, values)| {
                    if values_in_ranges(values, ranges) {
                        Some(field_idx as usize)
                    } else {
                        None
                    }
                })
                .collect();

            (name, placements)
        })
        .collect();

    let mut final_placements: Vec<String> =
        (0..field_values.len()).map(|_| String::from("")).collect();

    while !possible_placements.is_empty() {
        let (next_placement, next_field) = possible_placements
            .iter()
            .find_map(|(name, placements)| {
                if placements.len() == 1 {
                    Some((placements[0], *name))
                } else {
                    None
                }
            })
            .unwrap();

        final_placements[next_placement] = next_field.clone();

        possible_placements.remove(next_field);

        for placements in possible_placements.values_mut() {
            placements.retain(|&x| x != next_placement);
        }
    }

    final_placements
        .iter()
        .enumerate()
        .filter_map(|(idx, field_name)| {
            if field_name.starts_with("departure") {
                Some(data.your_ticket[idx])
            } else {
                None
            }
        })
        .product()
}

/// Transpose tickets into vectors of values per field.
fn get_field_values(tickets: &[Ticket], fields: &Fields) -> Vec<Vec<u64>> {
    let valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|x| fields.values().flatten().any(|range| range.contains(&x)))
        })
        .collect();

    (0..valid_tickets[0].len())
        .map(|idx| valid_tickets.iter().map(|x| x[idx]).collect())
        .collect()
}

fn values_in_ranges(values: &[u64], ranges: &[Range<u64>]) -> bool {
    values.iter().all(|x| ranges.iter().any(|r| r.contains(&x)))
}
