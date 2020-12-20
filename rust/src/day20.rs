use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../inputs/day20.txt");

// TODO: How can I convert Vec here to a static 10-sized array? Should offer speed gains.
//
// I missed that tiles could rotate and flip. Be back later...

type TileID = u64;
// Columns<bool> (contained in Rows)
type TileRow = Vec<bool>;
// Tile ID -> Tile
type Tiles = HashMap<TileID, Vec<TileRow>>;
// One row/col of a tile.
type TileBorder = Vec<bool>;
// Tile Border -> ID of tiles that have it
type TileBorders = HashMap<TileBorder, Vec<TileID>>;

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

use Direction::*;

pub fn run() {
    let tiles = transform_input(INPUT);

    println!("Part 1: {}", part1(&tiles));
}

fn transform_input(input: &'static str) -> Tiles {
    input
        .split("\n\n")
        .filter_map(|block| {
            let mut lines = block.lines();

            // This is such a mess.
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
        .collect()
}

fn part1(tiles: &Tiles) -> u64 {
    let borders = map_borders(tiles);

    let corners: Vec<TileID> = tiles
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
        .collect();

    assert_eq!(corners.len(), 4);
    corners.iter().product()
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
