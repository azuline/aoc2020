use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/day08.txt");

type Instruction = (&'static str, i32);

pub fn run() {
    let instructions = transform_input(INPUT);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

fn transform_input(input: &'static str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let (op, arg) = x.splitn(2, ' ').next_tuple().unwrap();
            (op, arg.parse().unwrap())
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> i32 {
    run_handheld(instructions).unwrap_err()
}

fn run_handheld(instructions: &[Instruction]) -> Result<i32, i32> {
    let mut pc = 0;
    let mut acc = 0;
    let mut insts_ran: HashSet<usize> = HashSet::new();

    while pc < instructions.len() {
        if insts_ran.contains(&pc) {
            return Err(acc);
        } else {
            insts_ran.insert(pc);
        }

        let (op, arg) = instructions[pc];

        if op == "jmp" {
            // TODO: Wtf >____>
            pc = (pc as i32 + arg) as usize;
            continue;
        }

        if op == "acc" {
            acc += arg;
        }

        pc += 1;
    }

    Ok(acc)
}

fn part2(instructions: &[Instruction]) -> i32 {
    for idx in 0..instructions.len() {
        let with_flipped = make_flipped_copy_at(instructions, idx);

        if let Ok(ret) = run_handheld(&with_flipped) {
            return ret;
        }
    }

    panic!("No incorrect instruction found.");
}

fn make_flipped_copy_at(instructions: &[Instruction], idx: usize) -> Vec<Instruction> {
    let inst = instructions[idx];

    // TODO: Any way to do this without like, creating a new Vec?

    match inst {
        ("acc", _) => instructions.to_vec(),
        ("jmp", arg) => {
            let mut with_flipped = instructions.to_vec();
            with_flipped[idx] = ("nop", arg);
            with_flipped
        }
        ("nop", arg) => {
            let mut with_flipped = instructions.to_vec();
            with_flipped[idx] = ("jmp", arg);
            with_flipped
        }
        _ => panic!("Impossible."),
    }
}
