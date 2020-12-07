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


from collections import defaultdict

bags_map = defaultdict(list)
inv_map = defaultdict(list)


for line in lines:
    thing, rest = line.split(" contain ")
    thing = thing.rstrip("bags .")
    for x in rest.split(", "):
        num, type_ = x.split(" ", 1)
        try:
            num = int(num)
        except ValueError:
            continue
        type_ = type_.rstrip(" bags.")
        bags_map[type_].append((thing, num))
        inv_map[thing].append((type_, num))


from pprint import pprint

pprint(bags_map)
pprint(inv_map)


def part1():
    seen_colors = set(["shiny gold"])
    visited_colors = set()

    while seen_colors:
        print("NEW")
        x = seen_colors.pop()
        next_ = [x[0] for x in bags_map[x]]
        print(x)
        print(next_)
        for y in next_:
            if y not in visited_colors:
                seen_colors.add(y)

        visited_colors.add(x)

    return len(visited_colors) - 1


def part2():
    seen_colors = set([("shiny gold", 1)])
    visited_colors = set()
    count = 0

    while seen_colors:
        print("NEW")
        (x, x_amt) = seen_colors.pop()
        print(x)
        print(x_amt)
        nexts = inv_map[x]
        print(nexts)

        for y, y_amt in nexts:
            count += x_amt * y_amt
            seen_colors.add((y, x_amt * y_amt))

        visited_colors.add(x)

    return count


# print(part1())
print("----------")
print(part2())
