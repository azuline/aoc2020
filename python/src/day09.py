#!/usr/bin/env python

from pathlib import Path
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day09.txt"


def transform_input(input: str) -> List[int]:
    return [int(x) for x in input.strip().split("\n")]


def in_prev_25_sums(number: int, numbers: List[int]) -> bool:
    complements = set()

    for x in numbers:
        if x in complements:
            return True

        complements.add(number - x)

    return False


def part1(numbers: List[int]) -> int:
    for i, number in enumerate(numbers[25:]):
        if not in_prev_25_sums(number, numbers[i : i + 25]):
            return number

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
