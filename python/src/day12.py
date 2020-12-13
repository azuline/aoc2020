#!/usr/bin/env python

"""
Note: This is the dirty solution from racing at midnight.

Have not written a nicer one.
"""

from pathlib import Path
from typing import List, Tuple

INPUT_FILE = Path.cwd().parent / "inputs" / "day12.txt"

directions = {
    "E": (1, 0),
    "S": (0, -1),
    "W": (-1, 0),
    "N": (0, 1),
}

dir_list = ["E", "S", "W", "N"]
indices = {
    "E": 0,
    "S": 1,
    "W": 2,
    "N": 3,
}


def transform_input(input: str) -> List[Tuple[str, int]]:
    actions = []

    for x in input.splitlines():
        actions.append((x[0], int(x[1:])))

    return actions


def move(dir_, mag, x, y):
    dx, dy = directions[dir_]
    dx *= mag
    dy *= mag
    return x + dx, y + dy


def part1(actions):
    x, y = 0, 0
    direction = "E"

    for (action, val) in actions:
        if action == "F":
            x, y = move(direction, val, x, y)
        elif action in ["N", "S", "E", "W"]:
            x, y = move(action, val, x, y)
        elif action == "R":
            old_idx = indices[direction]
            new_idx = (old_idx + (val // 90)) % 4
            direction = dir_list[new_idx]
        elif action == "L":
            old_idx = indices[direction]
            new_idx = (old_idx - (val // 90)) % 4
            direction = dir_list[new_idx]

    return abs(x) + abs(y)


def part2(actions):
    x, y = 0, 0
    way_dx, way_dy = 10, 1
    direction = "E"

    for (action, val) in actions:
        if action == "F":
            x += way_dx * val
            y += way_dy * val
        elif action in ["N", "S", "E", "W"]:
            way_dx, way_dy = move(action, val, way_dx, way_dy)
        elif action == "R":
            for _ in range(val // 90):
                way_dx, way_dy = way_dy, -way_dx

            new_idx = (indices[direction] + (val // 90)) % 4
            direction = dir_list[new_idx]
        elif action == "L":
            for _ in range(val // 90):
                way_dx, way_dy = -way_dy, way_dx

            new_idx = (indices[direction] + (val // 90)) % 4
            direction = dir_list[new_idx]

    return abs(x) + abs(y)


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        actions = transform_input(f.read())

    print(f"Part 1: {part1(actions)}")
    print(f"Part 2: {part2(actions)}")
