use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day11.txt");

const EIGHT_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Place {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

type Coord = (i32, i32);
type Grid = HashMap<Coord, Place>;

pub fn run() {
    let grid = transform_input(INPUT);

    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
}

fn transform_input(input: &'static str) -> Grid {
    let mut grid: Grid = HashMap::new();

    for (x, row) in input.lines().enumerate() {
        for (y, elem) in row.chars().enumerate() {
            let place = match elem {
                '.' => Place::FLOOR,
                '#' => Place::OCCUPIED,
                'L' => Place::EMPTY,
                _ => panic!("Invalid element in initial grid."),
            };

            grid.insert((x as i32, y as i32), place);
        }
    }

    grid
}

fn part1(mut grid: Grid) -> usize {
    let calc_adjacent_occupied = |calc_grid: &Grid, (x, y): &Coord| -> usize {
        EIGHT_DIRECTIONS
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|c| calc_grid.get(&c) == Some(&Place::OCCUPIED))
            .count()
    };

    occupy_airplane(&mut grid, calc_adjacent_occupied, 4);

    count_occupied_seats(&grid)
}

fn part2(mut grid: Grid) -> usize {
    let calc_visible_occupied = |calc_grid: &Grid, (x, y): &Coord| -> usize {
        EIGHT_DIRECTIONS
            .iter()
            .map(|(dx, dy)| {
                let mut adj_x = *x + dx;
                let mut adj_y = *y + dy;

                while let Some(place) = calc_grid.get(&(adj_x, adj_y)) {
                    if *place != Place::FLOOR {
                        return *place == Place::OCCUPIED;
                    }

                    adj_x += dx;
                    adj_y += dy;
                }

                false
            })
            .filter(|&x| x)
            .count()
    };

    occupy_airplane(&mut grid, calc_visible_occupied, 5);

    count_occupied_seats(&grid)
}

fn occupy_airplane<F: Fn(&Grid, &Coord) -> usize>(
    grid: &mut Grid,
    calc_nearby_occupied: F,
    seats_to_unoccupy: usize,
) {
    let mut prev_grid = grid.clone();

    loop {
        let mut modified = false;

        for (coord, place) in prev_grid.iter() {
            let num_occupied = calc_nearby_occupied(&prev_grid, coord);

            if *place == Place::EMPTY && num_occupied == 0 {
                grid.insert(*coord, Place::OCCUPIED);
                modified = true;
            } else if *place == Place::OCCUPIED && num_occupied >= seats_to_unoccupy {
                grid.insert(*coord, Place::EMPTY);
                modified = true;
            }
        }

        if !modified {
            break;
        }

        for (coord, place) in grid.iter() {
            prev_grid.insert(*coord, *place);
        }
    }
}

fn count_occupied_seats(grid: &Grid) -> usize {
    grid.values().filter(|x| **x == Place::OCCUPIED).count()
}
