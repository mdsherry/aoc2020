#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

use std::collections::HashSet;

use Op::*;

impl Op {
    fn parse(s: &str) -> Op {
        match s {
            "nop" => Nop,
            "acc" => Acc,
            "jmp" => Jmp,
            _ => unimplemented!("No op '{}'", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instr {
    op: Op,
    arg: i32,
}

impl Instr {
    fn parse(s: &str) -> Instr {
        let mut it = s.split_ascii_whitespace();
        let op = Op::parse(it.next().unwrap());
        let arg: i32 = it.next().unwrap().parse().unwrap();
        Instr { op, arg }
    }
}

struct CPU {
    instrs: Vec<Instr>,
    pc: usize,
    acc: i32,
}

enum CpuErrors {
    OutOfBounds,
    NarrowOutOfBounds,
}

impl CPU {
    fn tick(&mut self) -> Result<(), CpuErrors> {
        if self.pc == self.instrs.len() {
            Err(CpuErrors::NarrowOutOfBounds)
        } else if self.pc > self.instrs.len() {
            Err(CpuErrors::OutOfBounds)
        } else {
            let instr = self.instrs[self.pc];
            match instr.op {
                Nop => self.pc += 1,
                Jmp => self.pc = ((self.pc as i64) + (instr.arg as i64)) as usize,
                Acc => {
                    self.acc += instr.arg;
                    self.pc += 1;
                }
            }
            Ok(())
        }
    }
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    let instrs: Vec<_> = INPUT.lines().map(|s| Instr::parse(s)).collect();
    'outer: for i in 0..instrs.len() {
        if instrs[i].op == Acc {
            continue;
        }
        let mut new_instrs = instrs.clone();
        new_instrs[i].op = if instrs[i].op == Jmp { Nop } else { Jmp };
        let mut cpu = CPU {
            instrs: new_instrs,
            pc: 0,
            acc: 0,
        };
        
       let mut seen = HashSet::new();
        while seen.insert(cpu.pc) {
            match cpu.tick() {
                Ok(_) => (),
                Err(CpuErrors::NarrowOutOfBounds) => {
                    println!("Success! acc = {}, i = {}", cpu.acc, i);
                    break 'outer;
                }
                Err(_) => continue 'outer
            }
        }
        continue;
    }
    
}
