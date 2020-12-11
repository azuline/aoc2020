#!/usr/bin/env python

"""
`./run.py` executes with the input currently in the clipboard. Use this for testing.
`./run.py s` executes with the real input and attempts a submission.
"""

import collections
import itertools
import re
import sys
from collections import defaultdict, deque
from functools import lru_cache, reduce
from itertools import combinations
from pathlib import Path
from pprint import pprint
from typing import List

import aocd  # type: ignore
import pyperclip  # type: ignore

import old
from old import DoesntHalt, Instruction, flatten, product, run_handheld

LIVE_RUN = len(sys.argv) > 1 and sys.argv[1].startswith("s")

# TODO: Remember to update these!
YEAR = 2020
DAY = 11

# Depending on whether this is a test run or not, get data from clipboard or from aocd.
data = aocd.get_data(day=DAY, year=YEAR) if LIVE_RUN else pyperclip.paste()

# EARLY VARIABLE DEFINITIONS
words: List[str] = [x.strip() for x in data.split()]
lines: List[str] = [x.strip() for x in data.rstrip("\n").split("\n")]
charmap: List[List[str]] = [list(line) for line in lines]

single_digits: List[int] = [int(x) for x in re.findall(r"\d", data)]
full_numbers: List[int] = [int(x) for x in re.findall(r"-?\d+", data)]

# RACING CODE STARTS HERE

pass


def part2():
    prev = [x.copy() for x in charmap]

    while True:
        from pprint import pprint

        for x, row in enumerate(prev):
            for y, char in enumerate(row):
                if char == ".":
                    continue
                elif char == "L":
                    for cx, cy in get_adjacent(prev, x, y):
                        if prev[cx][cy] == "#":
                            break
                    else:
                        charmap[x][y] = "#"
                elif char == "#":
                    num_occ = 0
                    for cx, cy in get_adjacent(prev, x, y):
                        if prev[cx][cy] == "#":
                            num_occ += 1

                    if num_occ >= 5:
                        charmap[x][y] = "L"

        if prev == charmap:
            break

        prev = [x.copy() for x in charmap]

    num_occ = 0

    for col in charmap:
        for x in col:
            if x == "#":
                num_occ += 1

    return num_occ


def get_adjacent(prev, bx, by):
    adjs = []
    for dx, dy in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (0, -1),
    ]:
        x = bx
        y = by
        while True:
            x += dx
            y += dy

            if x < 0 or y < 0 or x >= len(prev) or y >= len(prev[0]):
                break

            try:
                if prev[x][y] != ".":
                    adjs.append((x, y))
                    break
            except IndexError:
                break

    return adjs


# p1_answer = part1()
# print(f"Part 1:\n\n{p1_answer}\n")
# if p1_answer and LIVE_RUN:
#     aocd.submit(p1_answer, day=DAY, year=YEAR, part="a")

p2_answer = part2()
print(f"Part 2:\n\n{p2_answer}\n")
if p2_answer and LIVE_RUN:
    aocd.submit(p2_answer, day=DAY, year=YEAR, part="b")
