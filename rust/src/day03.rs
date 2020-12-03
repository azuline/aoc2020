static INPUT: &str = include_str!("../../inputs/day03.txt");

type Lines = Vec<Vec<char>>;

pub fn run() {
    let (width, lines) = transform_input(INPUT);

    println!("Part 1: {}", part1(width, &lines, 3, 1));
    println!("Part 2: {}", part2(width, &lines));
}

fn transform_input(input: &str) -> (usize, Lines) {
    let lines: Lines = input.lines().map(|x| x.chars().collect()).collect();
    let width = lines[0].len();
    (width, lines)
}

fn part1(width: usize, lines: &Lines, x: usize, y: usize) -> usize {
    lines
        .iter()
        .step_by(y)
        .enumerate()
        .filter(|(idx, line)| line[idx * x % width] == '#')
        .count()
}

fn part2(width: usize, lines: &Lines) -> usize {
    let slices = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slices
        .iter()
        .map(|&(x, y)| part1(width, lines, x, y))
        .product()
}
