use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day20.txt");

const MONSTER_SIZE: usize = 15;
const MONSTER_OFFSETS: [(i32, i32); MONSTER_SIZE] = [
    (0, 0),
    (1, 1),
    (4, 1),
    (5, 0),
    (6, 0),
    (7, 1),
    (10, 1),
    (11, 0),
    (12, 0),
    (13, 1),
    (16, 1),
    (17, 0),
    (18, -1),
    (18, 0),
    (19, 0),
];

type TileID = u64;
// Columns of a tile row.
type TileRow = Vec<bool>;
// Tile ID -> Tile
type Tiles = HashMap<TileID, Vec<TileRow>>;

// One row/col of a tile.
type TileBorder = Vec<bool>;
// Value type for TileBorders.
#[derive(Debug)]
struct TBValue {
    tile_id: TileID,
    direction: Direction,
    reversed: bool,
}
// Tile Border -> ID of tiles that have it
type TileBorders = HashMap<TileBorder, Vec<TBValue>>;

// A tile on the final grid, can be rotated/flipped.
#[derive(Clone, Copy, Debug)]
struct GridTile {
    tile_id: TileID,
    rot_ccws: u8,
    flip_y: bool,
}
// Column of a stitched together grid row.
type StitchedGridRow = Vec<bool>;

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

use Direction::*;

pub fn run() {
    let (tiles, borders) = transform_input(INPUT);

    println!("Part 1: {}", part1(&tiles, &borders));
    println!("Part 2: {}", part2(&tiles, &borders));
}

fn transform_input(input: &'static str) -> (Tiles, TileBorders) {
    let tiles = input
        .split("\n\n")
        .filter_map(|block| {
            let mut lines = block.lines();

            let id = lines
                .next()?
                .strip_prefix("Tile ")?
                .strip_suffix(':')?
                .parse()
                .ok()?;

            let tile = lines
                .map(|row| row.chars().map(|c| c == '#').collect())
                .collect();

            Some((id, tile))
        })
        .collect();

    let borders = map_borders(&tiles);

    (tiles, borders)
}

fn part1(tiles: &Tiles, borders: &TileBorders) -> u64 {
    get_corners(tiles, &borders).iter().product()
}

fn get_corners(tiles: &Tiles, borders: &TileBorders) -> Vec<TileID> {
    let corners: Vec<TileID> = tiles
        .iter()
        .filter(|(_, tile)| {
            let unmatched_edge_count = get_all_tile_borders(tile)
                .into_iter()
                .filter(|border| borders.get(&border.0).unwrap().len() == 1)
                .collect_vec()
                .len();

            // We are looking for 4 instead of 2 because of the flipped edges.
            unmatched_edge_count == 4
        })
        .map(|(id, _)| *id)
        .collect();

    assert_eq!(corners.len(), 4);
    corners
}

fn map_borders(tiles: &Tiles) -> TileBorders {
    let mut tile_borders: TileBorders = HashMap::new();

    for (&tile_id, tile) in tiles.iter() {
        for (border, direction, reversed) in get_all_tile_borders(tile) {
            tile_borders
                .entry(border)
                .or_insert_with(Vec::new)
                .push(TBValue {
                    tile_id,
                    direction,
                    reversed,
                });
        }
    }

    tile_borders
}

fn get_all_tile_borders(tile: &[TileRow]) -> Vec<(TileBorder, Direction, bool)> {
    let u_border = get_tile_border(tile, &UP);
    let r_border = get_tile_border(tile, &RIGHT);
    let d_border = get_tile_border(tile, &DOWN);
    let l_border = get_tile_border(tile, &LEFT);

    vec![
        (u_border.clone().into_iter().rev().collect(), UP, true),
        (r_border.clone().into_iter().rev().collect(), RIGHT, true),
        (d_border.clone().into_iter().rev().collect(), DOWN, true),
        (l_border.clone().into_iter().rev().collect(), LEFT, true),
        (u_border, UP, false),
        (r_border, RIGHT, false),
        (d_border, DOWN, false),
        (l_border, LEFT, false),
    ]
}

fn get_tile_border(tile: &[TileRow], direction: &Direction) -> TileBorder {
    match direction {
        UP => tile[0].clone(),
        RIGHT => tile.iter().map(|r| r[9]).collect(),
        DOWN => tile[9].clone(),
        LEFT => tile.iter().map(|r| r[0]).collect(),
    }
}

fn part2(tiles: &Tiles, borders: &TileBorders) -> usize {
    let starting_corner = get_starting_corner(tiles, &borders);
    let full_grid = build_full_grid(tiles, &borders, starting_corner);
    let num_true = count_true_in_grid(&full_grid);

    for grid_rot in generate_grid_rotations(full_grid) {
        let num_monsters = count_monsters(&grid_rot);

        if num_monsters > 0 {
            return num_true - MONSTER_SIZE * num_monsters;
        }
    }

    panic!("No loch-ness monsters found!");
}

/// The starting corner must be a "top left" corner. We take one of
/// the corners and rotate it until it fits this criteria.
fn get_starting_corner(tiles: &Tiles, borders: &TileBorders) -> GridTile {
    let tile_id = *get_corners(tiles, &borders).first().unwrap();
    let tile = tiles.get(&tile_id).unwrap();

    let rot_ccws = (0..4)
        .find(|rot| {
            let rotated_tile = rotate_tile(tile.clone(), *rot);

            vec![UP, LEFT].iter().all(|dir| {
                let border = get_tile_border(&rotated_tile, dir);
                borders.get(&border).unwrap().len() == 1
            })
        })
        .unwrap();

    GridTile {
        tile_id,
        rot_ccws,
        flip_y: false,
    }
}

/// Rotate the tile counterclockwise 90deg `rot` times.
/// Each rotation is accomplished by transposing and y-flipping.
fn rotate_tile(tile: Vec<TileRow>, rot: u8) -> Vec<TileRow> {
    (0..rot).fold(tile, |tile, _| {
        let transposed = (0..tile.len())
            .map(|row| (0..tile[0].len()).map(|col| tile[col][row]).collect_vec())
            .collect_vec();

        flip_y(transposed)
    })
}

fn flip_y(tile: Vec<TileRow>) -> Vec<TileRow> {
    tile.into_iter().rev().collect()
}

fn build_full_grid(
    tiles: &Tiles,
    borders: &TileBorders,
    starting_corner: GridTile,
) -> Vec<StitchedGridRow> {
    let grid_width_height = (tiles.len() as f64).sqrt() as usize;
    let mut grid_tiles: Vec<Vec<GridTile>> = Vec::new();

    for row in 0..grid_width_height {
        let mut cur_tile = match row {
            0 => starting_corner,
            row => {
                let above_tile = &grid_tiles[row - 1][0];
                find_bordering_tile_down(tiles, borders, above_tile)
            }
        };

        grid_tiles.push(vec![cur_tile]);

        for _ in 1..grid_width_height {
            cur_tile = find_bordering_tile_right(tiles, borders, &cur_tile);
            grid_tiles[row].push(cur_tile);
        }
    }

    stitch_tiles(grid_tiles, tiles)
}

fn find_bordering_tile_right(
    tiles: &Tiles,
    borders: &TileBorders,
    grid_tile: &GridTile,
) -> GridTile {
    let tile = get_and_orient_tile(tiles, grid_tile);
    let border = get_tile_border(&tile, &RIGHT);

    let next_tile = borders
        .get(&border)
        .unwrap()
        .iter()
        .find(|b| b.tile_id != grid_tile.tile_id)
        .unwrap();

    let (rot_ccws, flip_y) = match (&next_tile.direction, next_tile.reversed) {
        (&UP, false) => (1, true),
        (&UP, true) => (1, false),
        (&DOWN, false) => (3, false),
        (&DOWN, true) => (3, true),
        (&LEFT, false) => (0, false),
        (&LEFT, true) => (0, true),
        (&RIGHT, false) => (2, true),
        (&RIGHT, true) => (2, false),
    };

    GridTile {
        tile_id: next_tile.tile_id,
        rot_ccws,
        flip_y,
    }
}

fn find_bordering_tile_down(
    tiles: &Tiles,
    borders: &TileBorders,
    grid_tile: &GridTile,
) -> GridTile {
    let tile = get_and_orient_tile(tiles, grid_tile);
    let border = get_tile_border(&tile, &DOWN);

    let next_tile = borders
        .get(&border)
        .unwrap()
        .iter()
        .find(|b| b.tile_id != grid_tile.tile_id)
        .unwrap();

    let (rot_ccws, flip_y) = match (&next_tile.direction, next_tile.reversed) {
        (&UP, false) => (0, false),
        (&UP, true) => (2, true),
        (&DOWN, false) => (0, true),
        (&DOWN, true) => (2, false),
        (&LEFT, false) => (1, true),
        (&LEFT, true) => (3, false),
        (&RIGHT, false) => (1, false),
        (&RIGHT, true) => (3, true),
    };

    GridTile {
        tile_id: next_tile.tile_id,
        rot_ccws,
        flip_y,
    }
}

fn get_and_orient_tile(tiles: &Tiles, grid_tile: &GridTile) -> Vec<TileRow> {
    let mut tile = tiles.get(&grid_tile.tile_id).unwrap().clone();
    tile = rotate_tile(tile, grid_tile.rot_ccws);
    if grid_tile.flip_y {
        tile = flip_y(tile);
    }
    tile
}

fn stitch_tiles(grid_tiles: Vec<Vec<GridTile>>, tiles: &Tiles) -> Vec<StitchedGridRow> {
    let mut stitched_tiles: Vec<StitchedGridRow> = Vec::new();

    for row in grid_tiles {
        let mut new_rows: Vec<StitchedGridRow> =
            (0..8).map(|_| Vec::new()).collect_vec();

        for grid_tile in row {
            let tile = get_and_orient_tile(tiles, &grid_tile);

            for idx_row in 0..8 {
                for idx_col in 0..8 {
                    new_rows[idx_row].push(tile[idx_row + 1][idx_col + 1]);
                }
            }
        }

        stitched_tiles.extend(new_rows);
    }

    stitched_tiles
}

fn count_true_in_grid(grid: &[StitchedGridRow]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|c| **c).count() as u64)
        .sum::<u64>() as usize
}

fn generate_grid_rotations(grid: Vec<StitchedGridRow>) -> Vec<Vec<StitchedGridRow>> {
    (0..4)
        .flat_map(|rot| {
            vec![
                rotate_tile(grid.clone(), rot),
                flip_y(rotate_tile(grid.clone(), rot)),
            ]
        })
        .collect()
}

fn count_monsters(grid: &[StitchedGridRow]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| does_monster_exist_at(grid, (row_idx, col_idx)))
                .collect_vec()
        })
        .filter(|x| *x)
        .count()
}

fn does_monster_exist_at(grid: &[StitchedGridRow], (row, col): (usize, usize)) -> bool {
    MONSTER_OFFSETS.iter().all(|(dy, dx)| {
        if let Some(grid_row) = grid.get((row as i32 + dx) as usize) {
            if let Some(grid_val) = grid_row.get((col as i32 + dy) as usize) {
                return *grid_val;
            }
        }

        false
    })
}

fn _print_tile(tile: &[TileRow]) {
    println!(
        "{}\n",
        tile.iter()
            .map(|v| v
                .iter()
                .map(|x| if *x { "#" } else { "." })
                .collect_vec()
                .join(""))
            .collect_vec()
            .join("\n")
    );
}
