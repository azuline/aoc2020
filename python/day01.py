#!/usr/bin/env python

from pathlib import Path
from typing import List

INPUT_FILE = Path(__file__).parent.parent / "inputs" / "day01.txt"


def transform_input(input: str) -> List[int]:
    return [int(x) for x in input.split()]


def part1(numbers: List[int]) -> int:
    complements = {2020 - x for x in numbers}

    for x in numbers:
        if x in complements:
            return x * (2020 - x)

    raise Exception("No pair found.")


def part2(numbers: List[int]) -> int:
    complements = {2020 - x for x in numbers}

    for i, x in enumerate(numbers):
        for y in numbers[i:]:
            if x + y in complements:
                return x * y * (2020 - x - y)

    raise Exception("No triplet found.")


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
