#!/usr/bin/env python

from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Generator, List, Set

INPUT_FILE = Path.cwd().parent / "inputs" / "day08.txt"


class DoesntHalt(Exception):
    pass


class Operation(Enum):
    ACC = "acc"
    JMP = "jmp"
    NOP = "nop"


@dataclass(frozen=True)
class Instruction:
    operation: Operation
    argument: int

    def flip(self) -> "Instruction":
        if self.operation == Operation.JMP:
            return self.__class__(Operation.NOP, self.argument)
        if self.operation == Operation.NOP:
            return self.__class__(Operation.JMP, self.argument)

        raise Exception("You've botched up the flip :(")


def transform_input(input: str) -> List[Instruction]:
    instructions = []

    for line in input.strip().split("\n"):
        op, arg = line.split(" ", 1)
        instructions.append(Instruction(Operation(op), int(arg)))

    return instructions


def part1(instructions: List[Instruction]) -> int:
    try:
        return run_handheld(instructions)
    except DoesntHalt as e:
        return e.args[0]


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

        if instr.operation == Operation.JMP:
            index += instr.argument
            continue

        if instr.operation == Operation.ACC:
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
        if instr.operation != Operation.ACC:
            new_list = instructions.copy()
            new_list[i] = instr.flip()
            yield new_list


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        instructions = transform_input(f.read())

    print(f"Part 1: {part1(instructions)}")
    print(f"Part 2: {part2(instructions)}")
