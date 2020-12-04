#!/usr/bin/env python

import itertools
import re
from functools import reduce
from pathlib import Path

from aocd import submit  # type: ignore

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

words = [x.strip() for x in data.split() if x.strip()]
lines = [x.strip() for x in data.split("\n")]

try:
    numbers = [int(x) for x in re.findall(r"-?\d+", data)]
except ValueError:
    pass

# TODO: SOLUTION HERE
# NOTE: DONT RETURN FROM THESE FUNCTIONS UNTIL WE WANT TO SUBMIT

# SET P1_DONE TRUE AFTER P1 IS DONE


passports = []
passport = ""
for line in lines:
    if line:
        passport += " " + line
    else:
        passports.append(passport)
        passport = ""


def part1(p):
    phrases = ["ecl:", "pid:", "eyr:", "hcl:", "byr:", "iyr:", "hgt:"]
    return all(x in p for x in phrases)


def part2():
    cnt = 0
    for p in passports:
        if not part1(p):
            pass

        byr = False
        iyr = False
        eyr = False
        hgt = False
        hcl = False
        ecl = False
        pid = False
        words = p.split(" ")
        for w in words:
            w = w.strip()
            print('"', w, '"')
            try:
                if w.startswith("byr:") and 1920 <= int(w[4:]) <= 2002:
                    byr = True
                    print(byr)
                elif w.startswith("iyr:") and 2010 <= int(w[4:]) <= 2020:
                    iyr = True
                    print(iyr)
                elif w.startswith("eyr:") and 2020 <= int(w[4:]) <= 2030:
                    eyr = True
                    print(eyr)
                elif w.startswith("hgt:"):
                    if w.endswith("cm") and 150 <= int(w[4:-2]) <= 193:
                        hgt = True
                        print(hgt, "cm")
                    elif w.endswith("in") and 59 <= int(w[4:-2]) <= 76:
                        hgt = True
                        print(hgt, "in")
                elif (
                    w.startswith("hcl:#")
                    and len(w) == 11
                    and all(c in "abcdef0123456789" for c in w[5:])
                ):

                    hcl = True
                    print(hcl)
                elif w.startswith("ecl:") and w[4:] in {
                    "amb",
                    "blu",
                    "brn",
                    "gry",
                    "grn",
                    "hzl",
                    "oth",
                }:
                    ecl = True
                    print(ecl)
                elif w.startswith("pid:") and int(w[4:]) and len(w) == 13:
                    pid = True
                    print(pid)
            except Exception as e:
                print(e)
                pass

        if byr and iyr and eyr and hgt and hcl and ecl and pid:
            cnt += 1

    return cnt


# print(part1())
print("----------")
print(part2())
