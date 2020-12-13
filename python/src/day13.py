#!/usr/bin/env python

"""
Note: This is the dirty solution from racing at midnight.

Have not written a nicer one.
"""

from pathlib import Path
from functools import reduce
import itertools
from typing import List, Tuple

INPUT_FILE = Path.cwd().parent / "inputs" / "day13.txt"


# Tuple of (bus id, offset)
Bus = Tuple[int, int]


def transform_input(input: str) -> Tuple[int, List[Bus]]:
    lines = input.splitlines()
    leaving_time = int(lines[0])
    buses = [(int(x), i) for i, x in enumerate(lines[1].split(",")) if x != "x"]

    return (leaving_time, buses)


def part1(leaving_time, buses):
    next_arrival = {b: 0 for b, _ in buses}

    for count in itertools.count():
        for x, na in next_arrival.items():
            if na == count:
                if na >= leaving_time:
                    return (na - leaving_time) * x

                next_arrival[x] += x


def part2(buses):
    n = [x[0] for x in buses]
    a = [x[1] for x in buses]
    N = product(n)

    return chinese_remainder_theorem(n, N, a)


def product(list_):
    return reduce(lambda x, y: x * y, list_, 1)


def chinese_remainder_theorem(n, N, a):
    result = 0

    for (ni, ai) in zip(n, a):
        _, _, si = extended_gcd(ni, N // ni)
        result += ai * si * (N // ni)

    return (-result) % N


def extended_gcd(a, b):
    """
    Recursive solution hits max recursive depth...
    depressing state of affairs Python...
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


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        leaving_time, buses = transform_input(f.read())

    print(f"Part 1: {part1(leaving_time, buses)}")
    print(f"Part 2: {part2(buses)}")
