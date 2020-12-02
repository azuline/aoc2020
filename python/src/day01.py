#!/usr/bin/env python

from pathlib import Path
from itertools import combinations
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day01.txt"


def transform_input(input: str) -> List[int]:
    return [int(x) for x in input.split()]


def part1(numbers: List[int]) -> int:
    complements = set()

    for x in numbers:
        if x in complements:
            return x * (2020 - x)

        complements.add(2020 - x)

    raise Exception("No pair found.")


def part2(numbers: List[int]) -> int:
    complements = set()

    for x, y in combinations(numbers, 2):
        if x + y in complements:
            return x * y * (2020 - x - y)

        complements.add(2020 - x - y)

    raise Exception("No triplet found.")


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
