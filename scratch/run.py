#!/usr/bin/env python

import itertools
import re
from collections import defaultdict, deque
from functools import reduce
from pathlib import Path
from pprint import pprint
from typing import List


def product(list_):
    return reduce(lambda x, y: x * y, list_, 1)


def flatten(list_):
    return list(itertools.chain(*list_))


# FILE READING
input_file = Path(__file__).parent / "input"

with input_file.open("r") as f:
    data: str = f.read()

# EARLY VARIABLE DEFINITIONS

words: List[str] = [x.strip() for x in data.split()]
lines: List[str] = [x.strip() for x in data.rstrip("\n").split("\n")]
charmap = [list(line) for line in lines]

try:
    numbers: List[int] = [int(x) for x in re.findall(r"-?\d+", data)]
except ValueError:
    pass

# RACING CODE STARTS HERE


def create_sums(i):
    sums_of_prev_25 = set()
    for x, y in itertools.combinations(numbers[i : i + 25], 2):
        sums_of_prev_25.add(x + y)

    return sums_of_prev_25


def part1():
    sums_of_prev = create_sums(0)

    for i, x in enumerate(numbers[25:], start=1):
        if x not in sums_of_prev:
            return x
        sums_of_prev = create_sums(i)

    raise Exception


def part2():
    target = part1()

    cur_sum = numbers[0]
    bottom = 0
    top = 0

    while top < len(numbers):
        if cur_sum == target:
            break
        elif cur_sum > target:
            cur_sum -= numbers[bottom]
            bottom += 1
        else:
            top += 1
            cur_sum += numbers[top]

    assert cur_sum == target
    range = numbers[bottom:top]
    print(bottom, top)
    print(range)
    return min(range) + max(range)


pprint(part1())
print("----------")
pprint(part2())
