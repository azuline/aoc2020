#!/usr/bin/env python

import re
from pathlib import Path
from typing import Dict, List

Passport = Dict[str, str]

INPUT_FILE = Path.cwd().parent / "inputs" / "day04.txt"
ECL_COLORS = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

VALIDATORS = {
    "byr": lambda x: 1920 <= int(x) <= 2002,
    "iyr": lambda x: 2010 <= int(x) <= 2020,
    "eyr": lambda x: 2020 <= int(x) <= 2030,
    "hgt": lambda x: validate_hgt(x),
    "hcl": lambda x: re.match(r"^#[0-9a-f]{6}$", x),
    "ecl": lambda x: x in ECL_COLORS,
    "pid": lambda x: re.match(r"^\d{9}$", x),
}


def validate_hgt(x: str) -> bool:
    if x[-2:] == "cm":
        return 150 <= int(x[:-2]) <= 193
    if x[-2:] == "in":
        return 59 <= int(x[:-2]) <= 76
    return False


def transform_input(input: str) -> List[Passport]:
    passports = []

    for line in input.split("\n\n"):
        passport = {}
        for word in line.split():
            try:
                key, val = word.split(":", 1)
                passport[key] = val
            except ValueError:
                pass
        passports.append(passport)

    return passports


def part1(passports: List[Passport]) -> int:
    return sum(check_passport_keys(x) for x in passports)


def check_passport_keys(passport: Passport) -> bool:
    return all(key in passport for key in VALIDATORS.keys())


def part2(passports: List[Passport]) -> int:
    return sum(check_passport_full(x) for x in passports)


def check_passport_full(passport: Passport) -> bool:
    for key, validator in VALIDATORS.items():
        try:
            if not validator(passport[key]):
                return False
        except (ValueError, KeyError):
            return False

    return True


if __name__ == "__main__":
    with INPUT_FILE.open("r") as f:
        input = transform_input(f.read())

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")
