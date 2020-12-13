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
from itertools import combinations, count
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
DAY = 13

# Depending on whether this is a test run or not, get data from clipboard or from aocd.
data = aocd.get_data(day=DAY, year=YEAR) if LIVE_RUN else pyperclip.paste()

# EARLY VARIABLE DEFINITIONS
words: List[str] = [x.strip() for x in data.split()]
lines: List[str] = [x.strip() for x in data.rstrip("\n").split("\n")]
charmap: List[List[str]] = [list(line) for line in lines]

single_digits: List[int] = [int(x) for x in re.findall(r"\d", data)]
full_numbers: List[int] = [int(x) for x in re.findall(r"-?\d+", data)]

# RACING CODE STARTS HERE

first_min = int(lines[0])
bus_intervals = [int(x) for x in lines[1].split(",") if x != "x"]


def part1():
    count = 0
    next_arrival = {b: 0 for b in bus_intervals}
    while True:
        for x, na in next_arrival.items():
            if na == count:
                if na >= first_min:
                    return (na - first_min) * x

                next_arrival[x] += x

        count += 1


def chinese_remainder_theorem(n, N, a):
    result = 0

    for i, (ni, ai) in enumerate(zip(n, a)):
        _, _, si = extended_gcd(ni, N // ni)
        result += ai * si * (N // ni)

    return (-result) % N


def extended_gcd(a, b):
    """
    Recursive solution hits max recursive depth... depressing state of affairs
    Python...
    """
    x, y, u, v = 0, 1, 1, 0

    while a != 0:
        q, r = b // a, b % a
        m, n = x - u * q, y - v * q
        b, a, x, y, u, v, = (
            a,
            r,
            u,
            v,
            m,
            n,
        )

    return b, x, y


def part2():
    bus_intervals = [(int(x), i) for i, x in enumerate(lines[1].split(",")) if x != "x"]
    N = product([x for x, _ in bus_intervals])

    n = [x[0] for x in bus_intervals]
    a = [x[1] for x in bus_intervals]

    return chinese_remainder_theorem(n, N, a)


p1_answer = part1()
print(f"Part 1:\n\n{p1_answer}\n")
if p1_answer and LIVE_RUN:
    aocd.submit(p1_answer, day=DAY, year=YEAR, part="a")


p2_answer = part2()
print(f"Part 2:\n\n{p2_answer}\n")
if p2_answer and LIVE_RUN:
    aocd.submit(p2_answer, day=DAY, year=YEAR, part="b")
