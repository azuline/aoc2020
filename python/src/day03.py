#!/usr/bin/env python

from functools import reduce
from pathlib import Path
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day03.txt"


def transform_input(input: str) -> List[str]:
    return [x.rstrip() for x in input.split("\n") if x]


def part1(lines: List[str], x=3, y=1) -> int:
    width = len(lines[0])
    return sum(line[x * i % width] == "#" for i, line in enumerate(lines[::y]))


def part2(lines: List[str]) -> int:
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    return product([part1(lines, x, y) for x, y in slopes])


def product(list_: List[int]) -> int:
    return reduce(lambda x, y: x * y, list_, 1)


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
