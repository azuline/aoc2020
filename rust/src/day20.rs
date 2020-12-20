use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day20.txt");
const MONSTER_SIZE: u64 = 15;

type TileID = u64;
// Columns of a tile row.
type TileRow = Vec<bool>;
// Tile ID -> Tile
type Tiles = HashMap<TileID, Vec<TileRow>>;
// One row/col of a tile.
type TileBorder = Vec<bool>;
// Tile Border -> ID of tiles that have it
type TileBorders = HashMap<TileBorder, Vec<TileID>>;

// A tile on the final grid, can be rotated/flipped.
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
    tiles
        .iter()
        .filter(|(_, tile)| {
            let unmatched_edge_count = get_all_tile_borders(tile)
                .into_iter()
                .filter(|border| borders.get(border).unwrap().len() == 1)
                .collect_vec()
                .len();

            // We are looking for 4 instead of 2 because of the flipped edges.
            unmatched_edge_count == 4
        })
        .map(|(id, _)| *id)
        .collect()
}

fn map_borders(tiles: &Tiles) -> TileBorders {
    let mut tile_borders: TileBorders = HashMap::new();

    for (id, tile) in tiles.iter() {
        for border in get_all_tile_borders(tile) {
            tile_borders
                .entry(border)
                .or_insert_with(Vec::new)
                .push(*id);
        }
    }

    tile_borders
}

fn get_all_tile_borders(tile: &[TileRow]) -> Vec<TileBorder> {
    let borders: Vec<TileBorder> = vec![
        get_tile_border(tile, &UP),
        get_tile_border(tile, &RIGHT),
        get_tile_border(tile, &DOWN),
        get_tile_border(tile, &LEFT),
    ];

    borders
        .into_iter()
        .flat_map(|border| vec![border.clone().into_iter().rev().collect(), border])
        .collect()
}

fn get_tile_border(tile: &[TileRow], direction: &Direction) -> TileBorder {
    match direction {
        UP => tile[0].clone(),
        RIGHT => tile.iter().map(|r| r[9]).collect(),
        DOWN => tile[9].clone(),
        LEFT => tile.iter().map(|r| r[0]).collect(),
    }
}

fn part2(tiles: &Tiles, borders: &TileBorders) -> u64 {
    let starting_corner = get_starting_corner(tiles, &borders);
    let full_grid = build_full_grid(tiles, &borders, starting_corner);
    let num_true = count_true_in_grid(&full_grid);

    for grid_rot in generate_grid_rotations(&full_grid) {
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

/// Rotate the tile clockwise 90deg `rot` times.
/// Each rotation is accomplished by transposing the columns and rows.
fn rotate_tile(tile: Vec<TileRow>, rot: u8) -> Vec<TileRow> {
    (0..rot).fold(tile, |tile, _| {
        (0..tile.len())
            .map(|row| (0..tile[0].len()).map(|col| tile[col][row]).collect_vec())
            .collect_vec()
    })
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
                find_bordering_tile(tiles, borders, above_tile, DOWN)
            }
        };

        grid_tiles[row].push(cur_tile);

        for _ in 1..grid_width_height {
            cur_tile = find_bordering_tile(tiles, borders, &cur_tile, RIGHT);
            grid_tiles[row].push(cur_tile);
        }
    }

    stitch_tiles(grid_tiles, tiles)
}

fn find_bordering_tile(
    tiles: &Tiles,
    borders: &TileBorders,
    tile_id: &GridTile,
    direction: Direction,
) -> GridTile {
    0
}

fn stitch_tiles(grid_tiles: Vec<Vec<GridTile>>, tiles: &Tiles) -> Vec<StitchedGridRow> {
    vec![]
}

fn count_true_in_grid(grid: &[StitchedGridRow]) -> u64 {
    grid.iter()
        .map(|row| row.iter().filter(|c| **c).count() as u64)
        .sum()
}

fn generate_grid_rotations(grid: &[StitchedGridRow]) -> Vec<Vec<StitchedGridRow>> {
    vec![]
}

fn count_monsters(grid: &[StitchedGridRow]) -> u64 {
    0
}
