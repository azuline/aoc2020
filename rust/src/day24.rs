use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../inputs/day24.txt");

// Our coordinate system is
// https://www.redblobgames.com/grids/hexagons/#coordinates-cube

#[derive(Clone, PartialEq, Eq, Hash)]
enum Instruction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
use Instruction::*;

type Instructions = Vec<Instruction>;
type Coord = (i32, i32, i32);
// A collection of black coordinates.
type GridBlacks = HashSet<Coord>;

lazy_static! {
    static ref OFFSETS: HashMap<Instruction, Coord> = {
        [
            (E, (1, -1, 0)),
            (SE, (0, -1, 1)),
            (SW, (-1, 0, 1)),
            (W, (-1, 1, 0)),
            (NW, (0, 1, -1)),
            (NE, (1, 0, -1)),
        ]
        .iter()
        .cloned()
        .collect()
    };
    static ref DIRECTIONS: Vec<Coord> = OFFSETS.clone().into_values().collect();
}

pub fn run() {
    let instructions = transform_input(INPUT);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

fn transform_input(input: &'static str) -> Vec<Instructions> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &'static str) -> Instructions {
    let mut instructions = Vec::new();
    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        let next_instruction = match c {
            'e' => E,
            's' => match chars.next() {
                Some('e') => SE,
                Some('w') => SW,
                _ => panic!("Invalid direction."),
            },
            'w' => W,
            'n' => match chars.next() {
                Some('w') => NW,
                Some('e') => NE,
                _ => panic!("Invalid direction."),
            },
            _ => panic!("Invalid direction."),
        };

        instructions.push(next_instruction);
    }

    instructions
}

fn part1(instructions_list: &[Instructions]) -> usize {
    make_initial_grid(instructions_list).len()
}

fn make_initial_grid(instructions_list: &[Instructions]) -> GridBlacks {
    let mut blacks: GridBlacks = HashSet::new();

    for instructions in instructions_list {
        let coord = instructions.iter().fold((0, 0, 0), |coord, inst| {
            add_coords(&coord, OFFSETS.get(&inst).unwrap())
        });

        if blacks.contains(&coord) {
            blacks.remove(&coord);
        } else {
            blacks.insert(coord);
        }
    }

    blacks
}

fn add_coords((x, y, z): &Coord, (dx, dy, dz): &Coord) -> Coord {
    (x + dx, y + dy, z + dz)
}

fn part2(instructions_list: &[Instructions]) -> usize {
    let mut blacks = make_initial_grid(instructions_list);

    for _day in 1..101 {
        let to_flip: Vec<Coord> = get_affected_coords(&blacks)
            .into_iter()
            .filter(|&coord| {
                let black = blacks.contains(&coord);
                let num_adj_blacks = get_adj_blacks(&blacks, coord);

                if black {
                    num_adj_blacks == 0 || num_adj_blacks > 2
                } else {
                    num_adj_blacks == 2
                }
            })
            .collect();

        for coord in to_flip {
            if blacks.contains(&coord) {
                blacks.remove(&coord);
            } else {
                blacks.insert(coord);
            }
        }

        // if _day < 10 || _day % 10 == 0 {
        //     let num_blacks = count_black(grid.clone());
        //     println!("Day {:>3}: {:>4} blacks", _day, num_blacks);
        // }
    }

    blacks.len()
}

// This is inefficiently implemented, but who cares!
fn get_affected_coords(grid: &GridBlacks) -> Vec<Coord> {
    grid.iter()
        .flat_map(|&c| {
            let mut coords = get_neighbors_of(c);
            coords.push(c);
            coords
        })
        .unique()
        .collect()
}

fn get_adj_blacks(grid: &GridBlacks, coord: Coord) -> usize {
    get_neighbors_of(coord)
        .iter()
        .filter(|neighbor| grid.contains(&neighbor))
        .count()
}

#[cached]
fn get_neighbors_of(coord: Coord) -> Vec<Coord> {
    DIRECTIONS
        .iter()
        .map(|offset| add_coords(&coord, offset))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_neighbor_coords() {
        let expected: Vec<Coord> = DIRECTIONS.clone().into_iter().sorted().collect();
        let neighbors: Vec<Coord> =
            get_neighbors_of((0, 0, 0)).into_iter().sorted().collect();

        assert_eq!(neighbors, expected);
    }

    #[test]
    fn getting_adj_blacks() {
        let grid: GridBlacks = [(1, -1, 0), (0, -1, 1)].iter().cloned().collect();

        assert_eq!(get_adj_blacks(&grid, (0, 0, 0)), 2);
        assert_eq!(get_adj_blacks(&grid, (1, -2, 1)), 2);
        assert_eq!(get_adj_blacks(&grid, (2, -1, -1)), 1);
    }

    #[test]
    fn getting_affected_coords() {
        let grid: GridBlacks = [(0, 0, 0)].iter().cloned().collect();

        assert!(get_affected_coords(&grid).contains(&(0, 0, 0)));
    }
}
