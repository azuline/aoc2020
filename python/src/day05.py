#!/usr/bin/env python

from pathlib import Path
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day05.txt"


def transform_input(input: str) -> List[str]:
    return input.strip().split("\n")


def part1(passes: List[str]) -> int:
    return max(transform_to_costs(passes))


def transform_to_costs(passes: List[str]) -> List[int]:
    return [calculate_cost(x) for x in passes]


def calculate_cost(x: str) -> int:
    bin_ = x.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1")
    return int(bin_, 2)


def part2(passes: List[str]) -> int:
    costs = set(transform_to_costs(passes))
    for x in range(min(costs) + 1, max(costs) - 1):
        if x not in costs and x - 1 in costs and x + 1 in costs:
            return x

    raise Exception("Impossible!")


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
