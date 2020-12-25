static INPUT: &str = include_str!("../../inputs/day25.txt");

const INITIAL_SUBJECT_NUM: u64 = 7;

type Pubkeys = (u64, u64);

pub fn run() {
    let pubkeys = transform_input(INPUT);
    println!("Part 1: {}", part1(&pubkeys));
}

fn transform_input(input: &'static str) -> Pubkeys {
    let mut lines = input.lines();
    let card = lines.next().unwrap().parse().unwrap();
    let door = lines.next().unwrap().parse().unwrap();

    (card, door)
}

fn part1(&(pk1, pk2): &Pubkeys) -> u64 {
    transform(pk2, crack_loop_size(pk1, INITIAL_SUBJECT_NUM))
}

fn crack_loop_size(pubkey: u64, subject_number: u64) -> usize {
    let mut privkey = 1;

    for iter in 1.. {
        privkey = (privkey * subject_number) % 20201227;
        if privkey == pubkey {
            return iter;
        }
    }

    panic!("never happening");
}

// tfw no modexp
fn transform(subject_number: u64, loop_size: usize) -> u64 {
    (0..loop_size).fold(1, |value, _| (value * subject_number) % 20201227)
}
