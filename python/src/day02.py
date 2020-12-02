#!/usr/bin/env python

import re
from collections import namedtuple
from pathlib import Path
from typing import List

INPUT_FILE = Path.cwd().parent / "inputs" / "day02.txt"
POLICY_REGEX = re.compile(r"^(\d+)-(\d+) (.): (.+)$")

Policy = namedtuple("Policy", "low high char password")


def transform_input(input: str) -> List[Policy]:
    policies = []

    for line in input.split("\n"):
        if m := POLICY_REGEX.match(line):
            policies.append(Policy(int(m[1]), int(m[2]), m[3], m[4]))

    return policies


def part1(policies: List[Policy]) -> int:
    return sum(p.low <= p.password.count(p.char) <= p.high for p in policies)


def part2(policies: List[Policy]) -> int:
    return sum(
        (p.password[p.low - 1] == p.char) ^ (p.password[p.high - 1] == p.char)
        for p in policies
    )


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
