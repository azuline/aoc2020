#!/usr/bin/env python

import itertools
from dataclasses import dataclass
from functools import reduce
from typing import Generator, List, Set

EIGHT_DIRECTIONS = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
]


def product(list_):
    return reduce(lambda x, y: x * y, list_, 1)


def flatten(list_):
    return list(itertools.chain(*list_))


class DoesntHalt(Exception):
    pass


@dataclass(frozen=True)
class Instruction:
    operation: str
    argument: int

    def flip(self) -> "Instruction":
        if self.operation == "jmp":
            return self.__class__("nop", self.argument)
        if self.operation == "nop":
            return self.__class__("jmp", self.argument)

        raise Exception("You've botched up the flip :(")


def transform_input(input: str) -> List[Instruction]:
    instructions = []

    for line in input.strip().split("\n"):
        op, arg = line.split(" ", 1)
        instructions.append(Instruction(op, int(arg)))

    return instructions


def run_handheld(instructions: List[Instruction]) -> int:
    acc = 0
    index = 0
    executed_indices: Set[int] = set()

    while index < len(instructions):
        # Check to see if we halt or not (visiting old index means no halt).
        if index in executed_indices:
            raise DoesntHalt(acc)
        else:
            executed_indices.add(index)

        instr = instructions[index]

        if instr.operation == "jmp":
            index += instr.argument
            continue

        if instr.operation == "acc":
            acc += instr.argument

        index += 1

    return acc


def part2(instructions: List[Instruction]) -> int:
    for instr_list in get_possible_correct_instr_lists(instructions):
        try:
            return run_handheld(instr_list)
        except DoesntHalt:
            pass

    raise Exception("Nothing halted!")


def get_possible_correct_instr_lists(
    instructions: List[Instruction],
) -> Generator[List[Instruction], None, None]:
    for i, instr in enumerate(instructions):
        if instr.operation == "acc":
            continue

        yield instructions[:i] + [instr.flip()] + instructions[i + 1 :]
