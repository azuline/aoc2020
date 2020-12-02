#!/usr/bin/env python

from pathlib import Path

# FILE READING

input_file = Path(__file__).parent / "input"

with input_file.open("r") as f:
    input = f.read()

# EARLY VARIABLE DEFINITIONS

words = input.split()
lines = input.split("\n")

try:
    numbers = [int(i) for i in input.split()]
except ValueError:
    pass

# SOLUTION

# Input transformation (if any is necessary.)

# Part 1

# Part 2
