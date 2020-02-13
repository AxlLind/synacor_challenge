use std::fs;
use itertools::Itertools;

const HLT: u16 = 0;  const SET: u16 = 1;
const PSH: u16 = 2;  const POP: u16 = 3;
const EQ:  u16 = 4;  const GT:  u16 = 5;
const JMP: u16 = 6;  const JT:  u16 = 7;
const JF:  u16 = 8;  const ADD: u16 = 9;
const MUL: u16 = 10; const MOD: u16 = 11;
const AND: u16 = 12; const OR:  u16 = 13;
const NOT: u16 = 14; const RD:  u16 = 15;
const WRT: u16 = 16; const CLL: u16 = 17;
const RET: u16 = 18; const OUT: u16 = 19;
const IN:  u16 = 20; const NOP: u16 = 21;

const OP_NAMES: [&str;22] = ["hlt", "set", "psh", "pop", " eq", " gt", "jmp", " jt", " jf", "add", "mul", "mod", "and", " or", "not", " rd", "wrt", "cll", "ret", "out", " in", "nop"];

fn read_program(path: &str) -> Vec<u16> {
  fs::read(path).unwrap()
    .iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect()
}

fn to_reg(i: u16) -> String {
  match i {
    0x0000..=0x7FFF => i.to_string(),
    0x8000..=0x8007 => format!("${}", i - 0x8000),
    _               => unreachable!()
  }
}

fn main() {
  let program = read_program("./challenge.bin");
  let mut i = 0;
  while i < 6100 {
    let pc = i;
    let opcode = program[i];
    let op = if opcode < 22 { OP_NAMES[opcode as usize] } else {""};
    let a = to_reg(program[i+1]);
    let b = to_reg(program[i+2]);
    let c = to_reg(program[i+3]);

    match opcode {
      HLT|RET|NOP              => { i += 1; println!("{:>4}: {}", pc, op); },
      PSH|POP|JMP|CLL|OUT|IN   => { i += 2; println!("{:>4}: {} {}", pc, op, a); },
      SET|JT|JF|NOT|RD|WRT     => { i += 3; println!("{:>4}: {} {} {}", pc, op, a, b); },
      EQ|GT|ADD|MUL|MOD|AND|OR => { i += 4; println!("{:>4}: {} {} {} {}", pc, op, a, b, c); },
      _                        => { i += 1; },
    };
  }
}
