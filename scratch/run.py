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


inputs = []
for line in lines:
    x, y = line.split(" ")
    inputs.append((x, int(y)))


def part1(instructions):
    acc = 0
    idx = 0
    executed = set()
    while idx < len(instructions):
        if idx in executed:
            # In original part 1, this was just return acc.
            return executed, acc
        executed.add(idx)
        x, y = instructions[idx]
        if x == "acc":
            acc += y
            idx += 1
        elif x == "jmp":
            idx += y
        else:
            idx += 1

    return "DONE", acc


def part2():
    instrs, _ = part1(inputs)
    jumps = [i for i in instrs if inputs[i][0] == "jmp"]
    for x in jumps:
        # NEEDED TO COPY THE LIST AHHHHHHHHHHHHHHHHHHHHh FUCK
        new_inputs = inputs.copy()
        new_inputs[x] = ("nop", inputs[x][1])
        ret, acc = part1(new_inputs)
        if ret == "DONE":
            return acc

    raise Exception("WTF")


print(part1(inputs))
print("----------")
print(part2())
