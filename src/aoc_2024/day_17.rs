use core::panic;

use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day17;

type Instruction = fn(u64, &mut [u64], &mut usize) -> Option<u64>;

fn get_combo_op(operand: u64, registers: &[u64]) -> u64 {
    match operand {
        0..=3 => operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("Invalid value for combo operator"),
    }
}

fn adv(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    let operand = get_combo_op(operand, registers);
    registers[0] /= 2_u64.pow(operand as u32);
    *pc += 2;

    None
}

fn bxl(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    registers[1] ^= operand;
    *pc += 2;

    None
}

fn bst(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    let operand = get_combo_op(operand, registers);
    registers[1] = operand % 8;
    *pc += 2;

    None
}

fn jnz(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    if registers[0] == 0 {
        *pc += 2;
    } else {
        *pc = operand as usize;
    }

    None
}

fn bxc(_operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    registers[1] ^= registers[2];
    *pc += 2;

    None
}

fn out(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    let val = get_combo_op(operand, registers) % 8;
    *pc += 2;

    Some(val)
}

fn bdv(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    let operand = get_combo_op(operand, registers);
    registers[1] = registers[0] / 2_u64.pow(operand as u32);
    *pc += 2;

    None
}

fn cdv(operand: u64, registers: &mut [u64], pc: &mut usize) -> Option<u64> {
    let operand = get_combo_op(operand, registers);
    registers[2] = registers[0] / 2_u64.pow(operand as u32);
    *pc += 2;

    None
}

fn parse_input(input : &String) -> (Vec<u64>, Vec<u64>) {
    let mut registers = Vec::new(); // To store register values
    let mut program = Vec::new(); // To store program sequence

    for line in input.lines() {
        let line = line.trim(); // Remove leading/trailing whitespace
        if line.starts_with("Register") {
            // Parse "Register X: Y"
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let value: u64 = parts[1].trim().parse().unwrap_or(0); // Parse the value
                registers.push(value);
            }
        } else if line.starts_with("Program") {
            // Parse "Program: X,Y,Z,..."
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                program = parts[1]
                    .trim()
                    .split(',')
                    .filter_map(|n| n.parse::<u64>().ok()) // Parse each number
                    .collect();
            }
        }
    }

    (registers, program)
}

fn solve(program: &Vec<u64>, reg_a: u64, pos: usize, recursive: bool) -> (bool, String) {
    let mut pc = 0;
    let mut output = Vec::new();

    let instruction_table: Vec<Instruction> = vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv];
    let mut registers = vec![reg_a, 0, 0];

    while pc < program.len() {
        let opcode = program[pc];
        let operand = program[pc + 1];
        let instruction = instruction_table[opcode as usize];

        if recursive && opcode == 3{
            return solve(program, registers[0], pos + 1, recursive);
        }

        match instruction(operand, &mut registers, &mut pc) {
            None => {},
            Some(val) => {
                output.push(val);

                if recursive {
                    if val != program[pos] {
                        return (false, "".to_string());
                    } else if pos == (program.len() - 1) {
                        return (true, "".to_string());
                    }
                }
            },
        }
    }
    
    (false, output.into_iter().join(",").to_string())
}

impl Plugin for AoC2024Day17 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(17, 2024);
        let puzzle = reader.load_puzzle();
       
        let (registers, program) = parse_input(&puzzle);
        let (_, part1) = solve(&program, registers[0], 0, false);
        println!("Part 1 solution {}", part1);

        let mut search: Vec<u64> = vec![0];
 
        for target in (0..program.len()).rev() {
            let mut next: Vec<u64> = Vec::new();
            println!("Search {:?}", search);

            for a in search.iter().flat_map(|a| (0..8).map(move |i| a + i)) {
                let (is_valid, _) = solve(&program, a, target, true);

                if is_valid {
                    if target == 0 {
                        return (0 as u64, a as u64);
                    }
                    next.push(a << 3);
                }
            }
            search = next;
        }

        (0, 0)
    }
}
