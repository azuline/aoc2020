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

// Column of a grid row.
type GridRow = Vec<bool>;

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
    let starting_corner = *get_corners(tiles, &borders).first().unwrap();
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

fn build_full_grid(
    tiles: &Tiles,
    borders: &TileBorders,
    starting_corner: u64,
) -> Vec<GridRow> {
    let mut grid_blocks: Vec<Vec<Vec<TileRow>>> = Vec::new();
    let grid_width_height = (tiles.len() as f64).sqrt() as u64;

    for row in 0..grid_width_height {
        for col in 0..grid_width_height {
            // do shit
        }
    }

    stitch_blocks(grid_blocks)
}

fn stitch_blocks(grid_blocks: Vec<Vec<Vec<TileRow>>>) -> Vec<GridRow> {
    vec![]
}

fn count_true_in_grid(grid: &[GridRow]) -> u64 {
    grid.iter()
        .map(|row| row.iter().filter(|c| **c).count() as u64)
        .sum()
}

fn generate_grid_rotations(grid: &[GridRow]) -> Vec<Vec<GridRow>> {
    vec![]
}

fn count_monsters(grid: &[GridRow]) -> u64 {
    0
}
