#!/usr/bin/env python

from itertools import chain
from pathlib import Path
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day06.txt"

AnswerGroup = List[str]


def transform_input(input: str) -> List[AnswerGroup]:
    return [x.split("\n") for x in input.strip().split("\n\n")]


def part1(groups: List[AnswerGroup]) -> int:
    return sum(len(set(chain(*x))) for x in groups)


def part2(groups: List[AnswerGroup]) -> int:
    return sum(len(set.intersection(*[set(y) for y in x])) for x in groups)


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
