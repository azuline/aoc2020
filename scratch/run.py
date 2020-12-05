#!/usr/bin/env python

import itertools
import re
from functools import reduce
from pathlib import Path

from aocd import submit  # type: ignore

YEAR = 2020
DAY = 3


def product(list_):
    return reduce(lambda x, y: x * y, list_, 1)


def flatten(list_):
    return list(itertools.chain(*list_))


# FILE READING
input_file = Path(__file__).parent / "input"

with input_file.open("r") as f:
    data = f.read()

# EARLY VARIABLE DEFINITIONS

words = [x.strip() for x in data.split()]
lines = [x.strip() for x in data.rstrip("\n").split("\n")]

try:
    numbers = [int(x) for x in re.findall(r"-?\d+", data)]
except ValueError:
    pass

# TODO: SOLUTION HERE


def part1():
    pass


def part2():
    pass


print(part1())
print("----------")
print(part2())
