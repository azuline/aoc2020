#!/usr/bin/env python

import itertools
import re
from functools import reduce
from pathlib import Path

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


def part1():
    ids = []
    for line in lines:
        low = 0
        high = 127
        collow = 0
        colhigh = 7
        print(line)
        for c in line:
            if c == "F":
                high = (high + low) // 2
            elif c == "B":
                low = (high + low) // 2 + 1
            elif c == "R":
                collow = (colhigh + collow) // 2 + 1
            elif c == "L":
                colhigh = (colhigh + collow) // 2
            print(low, high, collow, colhigh)

        seat_id = low * 8 + collow
        print(seat_id)
        ids.append(seat_id)

    return ids


def part2():
    ids = part1()
    print(sorted(ids))
    max_ = max(ids)
    low_ = min(ids)
    print(max_, low_)
    print("PRINTING")
    for i in range(low_, max_):
        if i not in ids and i - 1 in ids and i + 1 in ids:
            print(i)
            # return i

    return max_


print(part1())
print("----------")
print(part2())
