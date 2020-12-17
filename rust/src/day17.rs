use itertools::Itertools;
use std::collections::HashMap;

// Ugh this whole 3D/4D thing is a mess. I don't want to think about it.
//
// We're just doing dependency injection. 3D and 4D traits, wrapper class over the grid, differing
// functions split up in implementations of the traits.

static INPUT: &str = include_str!("../../inputs/day17.txt");
const THREE_ADJACENTS: [i32; 3] = [-1, 0, 1];

// (x, y, z, w)
type Coord = (i32, i32, i32, i32);

// Map of coords to active (true) / inactive (false).
type Grid = HashMap<Coord, bool>;

trait ConwayGrid {
    fn get_dimensions(&self) -> Dimensions;
}

impl ConwayGrid for HashMap<Coord, bool> {
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
}

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

    // TODO: How can I use iter() for this? Rather than do this crap vector stuff.
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

trait GridFns {
    fn get_mutating_coords(&self, grid: &Grid) -> Dimensions;
    fn get_adjacent_cubes(&self, coord: &Coord) -> Vec<Coord>;
}

#[derive(Clone)]
struct ThreeDimensionFns {}

impl GridFns for ThreeDimensionFns {
    fn get_mutating_coords(&self, grid: &Grid) -> Dimensions {
        let Dimensions {
            x: old_x,
            y: old_y,
            z: old_z,
            w: old_w,
        } = grid.get_dimensions();

        Dimensions {
            x: (old_x.0 - 1, old_x.1 + 1),
            y: (old_y.0 - 1, old_y.1 + 1),
            z: (old_z.0 - 1, old_z.1 + 1),
            w: old_w,
        }
    }

    fn get_adjacent_cubes(&self, (x, y, z, w): &Coord) -> Vec<Coord> {
        THREE_ADJACENTS
            .iter()
            .combinations_with_replacement(3)
            .filter(|adj| *adj[0] != 0 || *adj[1] != 0 || *adj[2] != 0)
            .flat_map(|c| c.into_iter().permutations(3).collect_vec())
            .unique()
            .map(|adj| (x + *adj[0], y + *adj[1], z + *adj[2], *w))
            .collect()
    }
}

#[derive(Clone)]
struct FourDimensionFns {}

impl GridFns for FourDimensionFns {
    fn get_mutating_coords(&self, grid: &Grid) -> Dimensions {
        let Dimensions {
            x: old_x,
            y: old_y,
            z: old_z,
            w: old_w,
        } = grid.get_dimensions();

        Dimensions {
            x: (old_x.0 - 1, old_x.1 + 1),
            y: (old_y.0 - 1, old_y.1 + 1),
            z: (old_z.0 - 1, old_z.1 + 1),
            w: (old_w.0 - 1, old_w.1 + 1),
        }
    }

    fn get_adjacent_cubes(&self, (x, y, z, w): &Coord) -> Vec<Coord> {
        THREE_ADJACENTS
            .iter()
            .combinations_with_replacement(4)
            .filter(|adj| *adj[0] != 0 || *adj[1] != 0 || *adj[2] != 0 || *adj[3] != 0)
            .flat_map(|c| c.into_iter().permutations(4).collect_vec())
            .unique()
            .map(|adj| (x + *adj[0], y + *adj[1], z + *adj[2], w + *adj[3]))
            .collect()
    }
}

#[derive(Clone)]
struct Data<T: GridFns + Clone> {
    grid: Grid,
    fns: T,
}

impl<T: GridFns + Clone> Data<T> {
    fn get_mutating_coords(&self) -> Dimensions {
        self.fns.get_mutating_coords(&self.grid)
    }

    fn get_active_adjacent_count(&self, coord: &Coord) -> u32 {
        self.fns
            .get_adjacent_cubes(coord)
            .iter()
            .filter(|c| *self.grid.get(c).unwrap_or(&false))
            .count() as u32
    }

    fn count_active(&self) -> u32 {
        self.grid
            .get_dimensions()
            .into_iter()
            .filter(|c| *self.grid.get(c).unwrap_or(&false))
            .count() as u32
    }
}

pub fn run() {
    let grid = transform_input(INPUT);

    let data_3d = Data {
        grid: grid.clone(),
        fns: ThreeDimensionFns {},
    };

    println!("Part 1: {}", six_rounds(data_3d));

    let data_4d = Data {
        grid,
        fns: FourDimensionFns {},
    };
    println!("Part 2: {}", six_rounds(data_4d));
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

fn six_rounds<T: GridFns + Clone>(mut data: Data<T>) -> u32 {
    let mut old_data = data.clone();

    for _ in 0..6 {
        for coord in old_data.get_mutating_coords() {
            let active_adjacent = old_data.get_active_adjacent_count(&coord);
            let active = *old_data.grid.get(&coord).unwrap_or(&false);

            if active && !(active_adjacent == 2 || active_adjacent == 3) {
                data.grid.insert(coord, false);
            } else if !active && active_adjacent == 3 {
                data.grid.insert(coord, true);
            }
        }

        old_data = data.clone();
    }

    data.count_active()
}
