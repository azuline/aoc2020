#!/usr/bin/env python

from copy import deepcopy
from pathlib import Path
from typing import Callable, Generator, List, Tuple

INPUT_FILE = Path.cwd().parent / "inputs" / "day11.txt"
EIGHT_DIRECTIONS = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
]

Grid = List[List[str]]
Coord = Tuple[int, int]


def transform_input(input: str) -> Grid:
    return [list(line) for line in input.rstrip("\n").split("\n")]


def part1(grid: Grid) -> int:
    return simulate_grid(
        grid,
        get_num_occupied=get_adjacent_num_occupied,
        num_occupied_for_move=4,
    )


def get_adjacent_num_occupied(grid: Grid, x: int, y: int) -> int:
    count = 0

    for dx, dy in EIGHT_DIRECTIONS:
        adj_x = x + dx
        adj_y = y + dy

        if 0 <= adj_x < len(grid) and 0 <= adj_y < len(grid[0]):
            if grid[adj_x][adj_y] == "#":
                count += 1

    return count


def part2(grid: Grid) -> int:
    return simulate_grid(
        grid,
        get_num_occupied=get_visible_num_occupied,
        num_occupied_for_move=5,
    )


def get_visible_num_occupied(grid: Grid, x: int, y: int) -> int:
    count = 0

    for dx, dy in EIGHT_DIRECTIONS:
        adj_x = x
        adj_y = y

        while True:
            adj_x += dx
            adj_y += dy

            if not (0 <= adj_x < len(grid) and 0 <= adj_y < len(grid[0])):
                break

            if grid[adj_x][adj_y] == "L":
                break

            if grid[adj_x][adj_y] == "#":
                count += 1
                break

    return count


def simulate_grid(
    grid: Grid,
    get_num_occupied: Callable,
    num_occupied_for_move: int,
) -> int:
    prev_grid = deepcopy(grid)

    while True:
        for x, y, elem in _get_grid_coords(prev_grid):
            num_occupied = get_num_occupied(prev_grid, x, y)

            if elem == "L" and not num_occupied:
                grid[x][y] = "#"
            elif elem == "#" and num_occupied >= num_occupied_for_move:
                grid[x][y] = "L"

        if prev_grid == grid:
            break

        prev_grid = deepcopy(grid)

    return count_occupied_seats(grid)


def _get_grid_coords(grid: Grid) -> Generator[Tuple[int, int, str], None, None]:
    for x, row in enumerate(grid):
        for y, elem in enumerate(row):
            yield x, y, elem


def count_occupied_seats(grid: Grid) -> int:
    return sum(x == "#" for row in grid for x in row)


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        grid = transform_input(f.read())

    print(f"Part 1: {part1(deepcopy(grid))}")
    print(f"Part 2: {part2(grid)}")
