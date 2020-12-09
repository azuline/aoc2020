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


def part1():
    pass


def part2():
    pass


pprint(part1())
print("----------")
pprint(part2())
