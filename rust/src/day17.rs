use itertools::Itertools;
use std::collections::HashMap;

// Ugh this whole 3D/4D thing is a mess. I don't want to think about it.

static INPUT: &str = include_str!("../../inputs/day17.txt");
const THREE_ADJACENTS: [i32; 3] = [-1, 0, 1];

// (x, y, z, w)
type Coord = (i32, i32, i32, i32);

// Dimensions are `[min, max)` (max-val is exclusive).
#[derive(Clone)]
struct Dimensions {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    w: (i32, i32),
}

impl IntoIterator for Dimensions {
    type Item = Coord;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut coords = Vec::new();

        for x in self.x.0..self.x.1 {
            for y in self.y.0..self.y.1 {
                for z in self.z.0..self.z.1 {
                    for w in self.w.0..self.w.1 {
                        coords.push((x, y, z, w));
                    }
                }
            }
        }

        coords.into_iter()
    }
}

// Map of coords to active (true) / inactive (false).
type Grid = HashMap<Coord, bool>;

trait ConwayGrid {
    fn get_mutating_coords_3d(&self) -> Dimensions;
    fn get_mutating_coords_4d(&self) -> Dimensions;
    fn get_dimensions(&self) -> Dimensions;
    fn get_active_adjacent_count_3d(&self, coord: &Coord) -> u32;
    fn get_active_adjacent_count_4d(&self, coord: &Coord) -> u32;
    fn count_active(&self) -> u32;
}

impl ConwayGrid for HashMap<Coord, bool> {
    fn get_mutating_coords_3d(&self) -> Dimensions {
        let Dimensions {
            x: old_x,
            y: old_y,
            z: old_z,
            w: old_w,
        } = self.get_dimensions();

        Dimensions {
            x: (old_x.0 - 1, old_x.1 + 1),
            y: (old_y.0 - 1, old_y.1 + 1),
            z: (old_z.0 - 1, old_z.1 + 1),
            w: old_w,
        }
    }

    fn get_mutating_coords_4d(&self) -> Dimensions {
        let Dimensions {
            x: old_x,
            y: old_y,
            z: old_z,
            w: old_w,
        } = self.get_dimensions();

        Dimensions {
            x: (old_x.0 - 1, old_x.1 + 1),
            y: (old_y.0 - 1, old_y.1 + 1),
            z: (old_z.0 - 1, old_z.1 + 1),
            w: (old_w.0 - 1, old_w.1 + 1),
        }
    }

    fn get_dimensions(&self) -> Dimensions {
        let keys = self.keys().collect_vec();
        let x_min = keys.iter().map(|c| c.0).min().unwrap();
        let x_max = keys.iter().map(|c| c.0).max().unwrap() + 1;
        let y_min = keys.iter().map(|c| c.1).min().unwrap();
        let y_max = keys.iter().map(|c| c.1).max().unwrap() + 1;
        let z_min = keys.iter().map(|c| c.2).min().unwrap();
        let z_max = keys.iter().map(|c| c.2).max().unwrap() + 1;
        let w_min = keys.iter().map(|c| c.3).min().unwrap();
        let w_max = keys.iter().map(|c| c.3).max().unwrap() + 1;

        Dimensions {
            x: (x_min, x_max),
            y: (y_min, y_max),
            z: (z_min, z_max),
            w: (w_min, w_max),
        }
    }

    fn get_active_adjacent_count_3d(&self, coord: &Coord) -> u32 {
        get_adjacent_cubes_3d(coord)
            .iter()
            .filter(|c| *self.get(c).unwrap_or(&false))
            .count() as u32
    }

    fn get_active_adjacent_count_4d(&self, coord: &Coord) -> u32 {
        get_adjacent_cubes_4d(coord)
            .iter()
            .filter(|c| *self.get(c).unwrap_or(&false))
            .count() as u32
    }

    fn count_active(&self) -> u32 {
        self.get_dimensions()
            .into_iter()
            .filter(|c| *self.get(c).unwrap_or(&false))
            .count() as u32
    }
}

/// Given a coordinate in the 3D grid, return the 26 adjacent coordinates.
fn get_adjacent_cubes_3d((x, y, z, w): &Coord) -> Vec<Coord> {
    THREE_ADJACENTS
        .iter()
        .combinations_with_replacement(3)
        .filter(|adj| *adj[0] != 0 || *adj[1] != 0 || *adj[2] != 0)
        .flat_map(|c| c.into_iter().permutations(3).collect_vec())
        .unique()
        .map(|adj| (x + *adj[0], y + *adj[1], z + *adj[2], *w))
        .collect()
}

/// Given a coordinate in the 3D grid, return the 26 adjacent coordinates.
fn get_adjacent_cubes_4d((x, y, z, w): &Coord) -> Vec<Coord> {
    THREE_ADJACENTS
        .iter()
        .combinations_with_replacement(4)
        .filter(|adj| *adj[0] != 0 || *adj[1] != 0 || *adj[2] != 0 || *adj[3] != 0)
        .flat_map(|c| c.into_iter().permutations(4).collect_vec())
        .unique()
        .map(|adj| (x + *adj[0], y + *adj[1], z + *adj[2], w + *adj[3]))
        .collect()
}

pub fn run() {
    let grid = transform_input(INPUT);

    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
}

fn transform_input(input: &'static str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, symbol)| ((x as i32, y as i32, 0, 0), symbol == '#'))
                .collect_vec()
        })
        .collect()
}

fn part1(mut grid: Grid) -> u32 {
    let mut old_grid = grid.clone();

    for _ in 0..6 {
        for coord in grid.get_mutating_coords_3d() {
            let active_adjacent = old_grid.get_active_adjacent_count_3d(&coord);
            let active = *old_grid.get(&coord).unwrap_or(&false);

            if active && !(active_adjacent == 2 || active_adjacent == 3) {
                grid.insert(coord, false);
            } else if !active && active_adjacent == 3 {
                grid.insert(coord, true);
            }
        }

        old_grid = grid.clone();
    }

    grid.count_active()
}

fn part2(mut grid: Grid) -> u32 {
    let mut old_grid = grid.clone();

    for _ in 0..6 {
        for coord in grid.get_mutating_coords_4d() {
            let active_adjacent = old_grid.get_active_adjacent_count_4d(&coord);
            let active = *old_grid.get(&coord).unwrap_or(&false);

            if active && !(active_adjacent == 2 || active_adjacent == 3) {
                grid.insert(coord, false);
            } else if !active && active_adjacent == 3 {
                grid.insert(coord, true);
            }
        }

        old_grid = grid.clone();
    }

    grid.count_active()
}
