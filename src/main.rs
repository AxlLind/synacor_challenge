use std::fs;
use std::io::Result;
use itertools::Itertools;
use easy_io::InputReader;

mod cpu;
use cpu::{CPU, ExitCode};

fn read_program(path: &str) -> Result<Vec<u16>> {
  let buf = fs::read(path)?;
  let program = buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect();
  Ok(program)
}

fn main() -> Result<()> {
  let program = read_program("./challenge.bin")?;
  let mut input = InputReader::new();
  let mut cpu = CPU::new(&program);
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => cpu.push_str(&input.next_line()),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
