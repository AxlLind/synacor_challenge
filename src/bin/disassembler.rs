use std::fs;
use std::io::{Result, Write};
use itertools::Itertools;
use easy_io::OutputWriter;

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

const OP_NAMES: [&str;22] = [
  "hlt", "set", "psh", "pop", "eq ", "gt ", "jmp", "jt ",
  "jf ", "add", "mul", "mod", "and", "or ", "not", "rd ",
  "wrt", "cll", "ret", "out", "in ", "nop",
];

fn read_program(path: &str) -> Result<Vec<u16>> {
  let buf = fs::read(path)?;
  let program = buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect::<Vec<_>>();
  Ok(program)
}

fn make_readable(i: u16) -> String {
  match i {
    0x0000..=0x7FFF => format!("{:#x}", i),
    0x8000..=0x8007 => format!("${}", i - 0x8000),
    _               => panic!()
  }
}

fn main() -> Result<()> {
  let program = read_program("files/challenge.bin")?;
  let mut out = OutputWriter::new();

  let mut i = 0;
  while i < 6100 {
    let pc = i;
    let opcode = program[i];
    let op = OP_NAMES.get(opcode as usize).unwrap_or(&"");
    let a = make_readable(program[i+1]);
    let b = make_readable(program[i+2]);
    let c = make_readable(program[i+3]);

    match opcode {
      HLT|RET|NOP              => { i += 1; writeln!(out, "{:#06x}: {}", pc, op)?; },
      PSH|POP|JMP|CLL|OUT|IN   => { i += 2; writeln!(out, "{:#06x}: {} {}", pc, op, a)?; },
      SET|JT|JF|NOT|RD|WRT     => { i += 3; writeln!(out, "{:#06x}: {} {} {}", pc, op, a, b)?; },
      EQ|GT|ADD|MUL|MOD|AND|OR => { i += 4; writeln!(out, "{:#06x}: {} {} {} {}", pc, op, a, b, c)?; },
      _                        => { i += 1; },
    };
  }

  Ok(())
}
