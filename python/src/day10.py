#!/usr/bin/env python

from collections import defaultdict
from pathlib import Path
from typing import Generator, List

INPUT_FILE = Path.cwd().parent / "inputs" / "day10.txt"


def transform_input(input: str) -> List[int]:
    numbers = sorted([int(x) for x in input.splitlines()])
    return [0, *numbers, numbers[-1] + 3]


def part1(numbers: List[int]) -> int:
    diffs = [numbers[i] - numbers[i - 1] for i in range(1, len(numbers))]
    return diffs.count(1) * diffs.count(3)


def part2(numbers: List[int]) -> int:
    num_paths = defaultdict(int)
    num_paths[0] = 1

    for window in windows(numbers, 4):
        for prev in window[:-1]:
            if window[-1] - prev <= 3:
                num_paths[window[-1]] += num_paths[prev]

    return num_paths[numbers[-1]]


def windows(list_: List, size: int) -> Generator[List, None, None]:
    for i in range(len(list_) + 1):
        yield list_[max(0, i - size) : i]


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        numbers = transform_input(f.read())

    print(f"Part 1: {part1(numbers)}")
    print(f"Part 2: {part2(numbers)}")
