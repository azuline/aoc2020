#!/usr/bin/env python

import re
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Set, Tuple

INPUT_FILE = Path.cwd().parent / "inputs" / "day07.txt"

ToContaining = Dict[str, List[str]]
FromContaining = Dict[str, List[Tuple[str, int]]]


def transform_input(input: str) -> Tuple[ToContaining, FromContaining]:
    to_containing = defaultdict(list)
    from_containing = defaultdict(list)

    for line in input.splitlines():
        parent_bag, rest = line.split(" bags contain ", 1)

        for num_bags, child_bag in re.findall(r"(\d+) (.+?) bag", rest):
            to_containing[child_bag].append(parent_bag)
            from_containing[parent_bag].append((child_bag, int(num_bags)))

    return to_containing, from_containing


def part1(bags_map: ToContaining) -> int:
    discovered = ["shiny gold"]
    visited: Set[str] = set()

    while discovered:
        bag = discovered.pop(0)
        discovered.extend(bags_map[bag])
        visited.add(bag)

    return len(visited) - 1


def part2(bags_map: FromContaining) -> int:
    discovered = [("shiny gold", 1)]
    total_bags = 0

    while discovered:
        bag, amount = discovered.pop(0)
        discovered.extend([(x, amount * x_amt) for x, x_amt in bags_map[bag]])
        total_bags += amount

    return total_bags - 1


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        to_containing, from_containing = transform_input(f.read())

    print(f"Part 1: {part1(to_containing)}")
    print(f"Part 2: {part2(from_containing)}")
