use std::fs;
use std::io::{Result, Write};
use itertools::Itertools;
use easy_io::{InputReader, OutputWriter};

mod cpu;
use cpu::{CPU, ExitCode};

const INPUT_FILES: [&str; 3] = [
  "./inputs/fetch_coins.txt",
  "./inputs/coin_order.txt",
  "./inputs/after_coins.txt",
];

fn read_program(path: &str) -> Result<Vec<u16>> {
  let buf = fs::read(path)?;
  let program = buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect();
  Ok(program)
}

fn fetch_inputs() -> Result<String> {
  INPUT_FILES.iter()
    .map(fs::read_to_string)
    .collect::<Result<Vec<_>>>()
    .map(|v| v.join(""))
}

fn main() -> Result<()> {
  let program = read_program("./challenge.bin")?;
  let mut cpu = CPU::new(&program);
  let mut input = InputReader::new();
  let mut out = OutputWriter::new();

  cpu.push_str(&fetch_inputs()?.trim());
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => {
        out.flush()?;
        cpu.push_str(&input.next_line());
      }
      ExitCode::Output(c) => out.print(c),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
