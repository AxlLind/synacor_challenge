use std::fs;
use std::io::{stdin, Result, Read};
use itertools::Itertools;

mod cpu;
use cpu::{CPU, ExitCode};

fn read_program(path: &str) -> Result<Vec<u16>> {
  let buf = fs::read(path)?;
  let program = buf.iter()
    .tuples()
    .map(|(&a,&b)| {
      let top = (b as u16) << 8;
      top | a as u16
    })
    .collect();
  Ok(program)
}

fn read_char() -> Result<u8> {
  stdin().lock().bytes().next().unwrap()
}

fn main() -> Result<()> {
  let program = read_program("./challenge.bin")?;
  let mut cpu = CPU::new(&program);
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => cpu.push_input(read_char()?),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
