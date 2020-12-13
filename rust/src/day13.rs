static INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Debug)]
struct Bus {
    id: i64,
    offset: i64,
}

pub fn run() {
    let (leaving_time, buses) = transform_input(INPUT);

    println!("Part 1: {}", part1(leaving_time, &buses));
    println!("Part 2: {}", part2(&buses));
}

fn transform_input(input: &'static str) -> (i64, Vec<Bus>) {
    let mut lines = input.lines();
    let leaving_time = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, x)| {
            Some(Bus {
                id: x.parse().ok()?,
                offset: offset as i64,
            })
        })
        .collect();

    (leaving_time, buses)
}

fn part1(leaving_time: i64, buses: &[Bus]) -> i64 {
    let (next_bus, waiting_time) = buses
        .iter()
        .map(|bus| (bus.id, bus.id - (leaving_time % bus.id)))
        .min_by_key(|x| x.1)
        .unwrap();

    next_bus * waiting_time
}

fn part2(buses: &[Bus]) -> i64 {
    let modulo: i64 = buses.iter().map(|bus| bus.id).product();

    let result: i64 = buses
        .iter()
        .map(|bus| {
            let b = modulo / bus.id;
            let (_, _, y) = extended_gcd(bus.id, b);
            bus.offset * y * b
        })
        .sum();

    (-result).rem_euclid(modulo)
}

#[allow(clippy::many_single_char_names)]
fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let mut x = 0;
    let mut y = 1;
    let mut u = 1;
    let mut v = 0;

    while a != 0 {
        let (q, r) = (b / a, b % a);
        let (m, n) = (x - u * q, y - v * q);
        b = a;
        a = r;
        x = v;
        y = v;
        u = m;
        v = n;
    }

    (b, x, y)
}
