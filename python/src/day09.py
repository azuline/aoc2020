#!/usr/bin/env python

from itertools import combinations
from pathlib import Path
from typing import List, Set

INPUT_FILE = Path.cwd().parent / "inputs" / "day09.txt"


def transform_input(input: str) -> List[int]:
    return [int(x) for x in input.strip().split("\n")]


def calculate_prev_25_sums(numbers: List[int]) -> Set[int]:
    return {x + y for x, y in combinations(numbers, 2)}


def part1(numbers: List[int]) -> int:
    sums = calculate_prev_25_sums(numbers[:25])

    for i, number in enumerate(numbers[25:], start=1):
        if number not in sums:
            return number

        sums = calculate_prev_25_sums(numbers[i : i + 25])

    raise Exception("Didn't find any weakness.")


def part2(numbers: List[int]) -> int:
    target = part1(numbers)
    contiguous_range = find_continguous_range(numbers, target)
    return min(contiguous_range) + max(contiguous_range)


def find_continguous_range(numbers: List[int], target: int) -> List[int]:
    current_sum = numbers[0]
    bottom = 0
    top = 0

    while top < len(numbers):
        if current_sum == target:
            return numbers[bottom:top]

        if current_sum > target:
            current_sum -= numbers[bottom]
            bottom += 1
        elif current_sum < target:
            top += 1
            current_sum += numbers[top]

    raise Exception("Didn't find a contiguous sum.")


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        numbers = transform_input(f.read())

    print(f"Part 1: {part1(numbers)}")
    print(f"Part 2: {part2(numbers)}")
