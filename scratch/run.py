#!/usr/bin/env python

import collections
import itertools
import re
from collections import defaultdict, deque
from functools import reduce
from itertools import combinations
from pathlib import Path
from pprint import pprint
from typing import List

import old
from old import DoesntHalt, Instruction, run_handheld


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

jolts = sorted(numbers)


def part1():
    num_1 = 0
    num_3 = 0

    prev = 0
    for j in jolts:
        if j - prev == 1:
            num_1 += 1
        elif j - prev == 3:
            num_3 += 1

        prev = j

    return num_1 * (num_3 + 1)


memo = {}


def part2(jolts, prev=0):
    if jolts == []:
        return 0
    if len(jolts) == 1:
        return 1

    cur = jolts[0]
    if cur in memo:
        return memo[cur]

    if len(jolts) > 2 and jolts[2] <= prev + 3:
        r = part2(jolts[1:], cur) + part2(jolts[2:], cur) + part2(jolts[3:], cur)
        memo[cur] = r
        return r
    if len(jolts) > 1 and jolts[1] <= prev + 3:
        r = part2(jolts[1:], cur) + part2(jolts[2:], cur)
        memo[cur] = r
        return r

    r = part2(jolts[1:], cur)
    memo[cur] = r
    return r


pprint(part1())
print("----------")
pprint(part2(jolts + [jolts[-1] + 3]))
