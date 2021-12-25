use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(Val),
    Add(Val, Val),
    Mul(Val, Val),
    Div(Val, Val),
    Mod(Val, Val),
    Eql(Val, Val),
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Lit(isize),
    Reg(usize),
}

impl Val {
    #[inline]
    fn read(&self, registers: &[isize; 4]) -> isize {
        match *self {
            Val::Lit(lit) => lit,
            Val::Reg(r) => registers[r],
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let ins = match &parts[..] {
            ["inp", a] => {
                let a = parse_value(a);
                Instruction::Input(a)
            }
            [kind, a, b] => {
                let a = parse_value(a);
                let b = parse_value(b);
                match *kind {
                    "add" => Instruction::Add(a, b),
                    "mul" => Instruction::Mul(a, b),
                    "div" => Instruction::Div(a, b),
                    "mod" => Instruction::Mod(a, b),
                    "eql" => Instruction::Eql(a, b),
                    _ => panic!("invalid instruction"),
                }
            }
            _ => panic!("invalid instruction"),
        };
        instructions.push(ins);
    }
    instructions
}

fn parse_value(val: &str) -> Val {
    if let Ok(lit) = val.parse::<isize>() {
        Val::Lit(lit)
    } else {
        let idx = match val.chars().next() {
            Some('w') => 0,
            Some('x') => 1,
            Some('y') => 2,
            Some('z') => 3,
            _ => panic!("bad register: {}", val),
        };
        Val::Reg(idx)
    }
}

fn part_one(instructions: Vec<Instruction>) -> usize {
    let mut partitioned = Vec::new();
    let mut partition = Vec::new();

    let mut peekable = instructions.iter().peekable();
    loop {
        match peekable.peek() {
            Some(Instruction::Input(_)) | None if !partition.is_empty() => {
                partitioned.push(partition);
                partition = Vec::new();
            }
            _ => {}
        }
        match peekable.next() {
            Some(ins) => partition.push(*ins),
            None => break,
        }
    }
    let input = "45989929946199"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();
    let out = exec(&instructions, &input, [0; 4]);
    println!("{:?}", out);
    // for i in all_inputs().take(100) {
    // println!("{:?}", i);
    // }
    0
}

fn exec(program: &[Instruction], input: &[isize], mut registers: [isize; 4]) -> [isize; 4] {
    let mut iter = input.iter();
    for ins in program {
        match ins {
            Instruction::Input(Val::Reg(r)) => {
                registers[*r] = *iter.next().expect("out of input");
            }
            Instruction::Add(Val::Reg(r), b) => {
                registers[*r] += b.read(&registers);
            }
            Instruction::Mul(Val::Reg(r), b) => {
                registers[*r] *= b.read(&registers);
            }
            Instruction::Div(Val::Reg(r), b) => {
                registers[*r] /= b.read(&registers);
            }
            Instruction::Mod(Val::Reg(r), b) => {
                registers[*r] %= b.read(&registers);
            }
            Instruction::Eql(Val::Reg(r), b) => {
                registers[*r] = (registers[*r] == b.read(&registers)) as isize;
            }
            _ => panic!("bad instruction: {:?}", ins),
        }
    }
    registers
}

fn part_two(_ins: Vec<Instruction>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
