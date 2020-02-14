use std::fs;
use std::io::Result;
use itertools::Itertools;
use easy_io::InputReader;
use cpu::{CPU, ExitCode};

fn read_program(path: &str) -> Result<Vec<u16>> {
  let buf = fs::read(path)?;
  let mut program = buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect::<Vec<_>>();

  // patches to the program:
  //   0x0209 - Skip the non-zero check of $7 in the test suite
  //   0x156D - Skip the expensive teleporter computation
  program[0x0209] = 8;
  program[0x156D] = 6;
  program[0x1571] = 21;
  program[0x1572] = 21;

  Ok(program)
}

fn main() -> Result<()> {
  let inputs = fs::read_to_string("./inputs.txt")?;
  let program = read_program("./challenge.bin")?;
  let mut cpu = CPU::new(&program);
  let mut input = InputReader::new();

  cpu.reg[7] = 25734; // computed in teleporter_setting.rs
  cpu.push_str(&inputs);
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => cpu.push_str(&input.next_line()),
      ExitCode::Output(c) => print!("{}", c),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
