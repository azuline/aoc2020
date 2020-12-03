#!/usr/bin/env python

import itertools
import re
from pathlib import Path

# FILE READING

input_file = Path(__file__).parent / "input"

with input_file.open("r") as f:
    data = f.read()

# EARLY VARIABLE DEFINITIONS

words = [x.strip() for x in data.split() if x.strip()]
lines = [x.strip() for x in data.split("\n") if x.strip()]

try:
    numbers = [int(x) for x in re.findall(r"-?\d+", data)]
except ValueError:
    pass

# SOLUTION HERE

idx = 0
width = len(lines[0])
mul = 1
adds = [1, 3, 5, 7]

for a in adds:
    cnt = 0
    idx = 0
    for x in lines:
        if x[idx] == "#":
            cnt += 1
        idx = (idx + a) % width
    print(cnt)
    mul = mul * cnt

idx = 0
cnt = 0
for i, x in enumerate(lines):
    if i % 2 != 0:
        continue
    print(i, x[idx], cnt)
    if x[idx] == "#":
        cnt += 1
    idx = (idx + 1) % width

print(cnt)
mul = mul * cnt

print(mul)
