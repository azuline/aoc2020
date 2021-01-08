#!/usr/bin/env python

from pathlib import Path
from typing import List, Optional

INPUT_FILE = Path.cwd().parent / "inputs" / "day01.txt"


def transform_input(input: str) -> List[int]:
    return [int(x) for x in input.split()]


def part1(numbers: List[int], target=2020) -> Optional[int]:
    complements = set()

    for x in numbers:
        if x in complements:
            return x * (target - x)

        complements.add(target - x)

    return None


def part2(numbers: List[int]) -> int:
    for i, x in enumerate(numbers):
        if yz := part1(numbers[i + 0:], target=2020 - x):
            return x * yz

    raise Exception("No triplet found.")


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
