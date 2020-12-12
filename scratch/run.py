#!/usr/bin/env python

"""
`./run.py` executes with the input currently in the clipboard. Use this for testing.
`./run.py s` executes with the real input and attempts a submission.
"""

import collections
import itertools
import math
import re
import sys
from collections import defaultdict, deque
from copy import copy, deepcopy
from functools import lru_cache, reduce
from itertools import combinations
from pathlib import Path
from pprint import pprint
from typing import List

import aocd  # type: ignore
import pyperclip  # type: ignore

import old
from old import (
    EIGHT_DIRECTIONS,
    DoesntHalt,
    Instruction,
    flatten,
    product,
    run_handheld,
)

LIVE_RUN = len(sys.argv) > 1 and sys.argv[1].startswith("s")

# TODO: Remember to update these!
YEAR = 2020
DAY = 12

# Depending on whether this is a test run or not, get data from clipboard or from aocd.
data = aocd.get_data(day=DAY, year=YEAR) if LIVE_RUN else pyperclip.paste()

# EARLY VARIABLE DEFINITIONS
words: List[str] = [x.strip() for x in data.split()]
lines: List[str] = [x.strip() for x in data.rstrip("\n").split("\n")]
charmap: List[List[str]] = [list(line) for line in lines]

single_digits: List[int] = [int(x) for x in re.findall(r"\d", data)]
full_numbers: List[int] = [int(x) for x in re.findall(r"-?\d+", data)]

# RACING CODE STARTS HERE

directions = {
    "E": (1, 0),
    "S": (0, -1),
    "W": (-1, 0),
    "N": (0, 1),
}

dir_list = ["E", "S", "W", "N"]
indices = {
    "E": 0,
    "S": 1,
    "W": 2,
    "N": 3,
}


actions = []

for x in lines:
    n = re.search(r"(\d+)", x)[1]
    actions.append((x.rstrip(n), int(n)))


def move(dir_, mag, x, y):
    dx, dy = directions[dir_]
    dx *= mag
    dy *= mag
    return x + dx, y + dy


def part1():
    x, y = 0, 0
    direction = "E"

    for (action, val) in actions:
        if action == "F":
            x, y = move(direction, val, x, y)
        elif action in ["N", "S", "E", "W"]:
            x, y = move(action, val, x, y)
        elif action == "R":
            old_idx = indices[direction]
            new_idx = (old_idx + (val // 90)) % 4
            direction = dir_list[new_idx]
        elif action == "L":
            old_idx = indices[direction]
            new_idx = (old_idx - (val // 90)) % 4
            direction = dir_list[new_idx]

        print(action, val, x, y, direction)

    return abs(x) + abs(y)


def part2():
    x, y = 0, 0
    way_dx, way_dy = 10, 1
    direction = "E"

    for (action, val) in actions:
        if action == "F":
            x += way_dx * val
            y += way_dy * val
        elif action in ["N", "S", "E", "W"]:
            way_dx, way_dy = move(action, val, way_dx, way_dy)
        elif action == "R":
            for _ in range(val // 90):
                way_dx, way_dy = way_dy, -way_dx

            new_idx = (indices[direction] + (val // 90)) % 4
            direction = dir_list[new_idx]
        elif action == "L":
            for _ in range(val // 90):
                way_dx, way_dy = -way_dy, way_dx

            new_idx = (indices[direction] + (val // 90)) % 4
            direction = dir_list[new_idx]

        print(action, val)
        print((x, y), (way_dx, way_dy), direction)

    return abs(x) + abs(y)


p1_answer = part1()
print(f"Part 1:\n\n{p1_answer}\n")
if p1_answer and LIVE_RUN:
    aocd.submit(p1_answer, day=DAY, year=YEAR, part="a")

p2_answer = part2()
print(f"Part 2:\n\n{p2_answer}\n")
if p2_answer and LIVE_RUN:
    aocd.submit(p2_answer, day=DAY, year=YEAR, part="b")
