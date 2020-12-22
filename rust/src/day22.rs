use std::cmp::Ordering;
use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/day22.txt");

type Deck = Vec<u64>;
type DecksHistory = HashSet<(Deck, Deck)>;

#[derive(Clone)]
struct Data {
    p1: Deck,
    p2: Deck,
}

#[derive(Debug)]
enum Player {
    ONE,
    TWO,
}

pub fn run() {
    let data = transform_input(INPUT);

    println!("Part 1: {}", part1(data.clone()));
    println!("Part 2: {}", part2(data));
}

fn transform_input(input: &'static str) -> Data {
    let (in_p1, in_p2) = input.split_once("\n\n").unwrap();

    Data {
        p1: parse_deck(in_p1),
        p2: parse_deck(in_p2),
    }
}

fn parse_deck(input: &str) -> Deck {
    input
        .split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(data: Data) -> u64 {
    let Data { mut p1, mut p2 } = data;

    while !p1.is_empty() && !p2.is_empty() {
        let p1_top = p1.remove(0);
        let p2_top = p2.remove(0);

        match p1_top.cmp(&p2_top) {
            Ordering::Greater => p1.extend_from_slice(&[p1_top, p2_top]),
            Ordering::Less => p2.extend_from_slice(&[p2_top, p1_top]),
            Ordering::Equal => panic!("There's a tie in my decks!"),
        }
    }

    let winning_deck = match p1.is_empty() {
        true => p2,
        false => p1,
    };

    calculate_deck_score(winning_deck)
}

fn calculate_deck_score(deck: Deck) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) as u64 * x)
        .sum()
}

fn part2(data: Data) -> u64 {
    let (_, winning_deck) = run_recursive_combat(data, 1);
    calculate_deck_score(winning_deck)
}

fn run_recursive_combat(data: Data, game_idx: u32) -> (Player, Deck) {
    let Data { mut p1, mut p2 } = data;
    let mut decks_history: DecksHistory = HashSet::new();
    let mut _round_idx = 1;

    while !p1.is_empty() && !p2.is_empty() {
        let deck_tup = (p1.clone(), p2.clone());

        if decks_history.contains(&deck_tup) {
            return (Player::ONE, p1);
        } else {
            decks_history.insert(deck_tup);
        }

        let p1_top = p1.remove(0);
        let p2_top = p2.remove(0);

        // println!("Round {} Game {}", &_round_idx, &game_idx);
        // println!("Drew: p1 {} | p2 {}", &p1_top, &p2_top);

        let winner = if p1_top > p1.len() as u64 || p2_top > p2.len() as u64 {
            // println!("Deciding game by value of card.");

            match p1_top.cmp(&p2_top) {
                Ordering::Greater => Player::ONE,
                Ordering::Less => Player::TWO,
                Ordering::Equal => panic!("There's a tie in my decks!"),
            }
        } else {
            // println!("Deciding game by recursive combat.");

            let (winner, _) = run_recursive_combat(
                Data {
                    p1: p1[..p1_top as usize].to_owned(),
                    p2: p2[..p2_top as usize].to_owned(),
                },
                game_idx + 1,
            );

            winner
        };

        // println!(
        //     "Round {} Game {} won by {:?}",
        //     &_round_idx, &game_idx, &winner
        // );

        match winner {
            Player::ONE => p1.extend_from_slice(&[p1_top, p2_top]),
            Player::TWO => p2.extend_from_slice(&[p2_top, p1_top]),
        };

        _round_idx += 1;
    }

    match p1.is_empty() {
        true => (Player::TWO, p2),
        false => (Player::ONE, p1),
    }
}
