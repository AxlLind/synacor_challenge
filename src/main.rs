use std::fs;
use std::io::Result;
use itertools::Itertools;
use easy_io::InputReader;

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

fn fetch_inputs() -> Result<Vec<String>> {
  INPUT_FILES.iter()
    .map(fs::read_to_string)
    .collect()
}

fn main() -> Result<()> {
  let program = read_program("./challenge.bin")?;
  let mut cpu = CPU::new(&program);
  let mut input = InputReader::new();

  for s in &fetch_inputs()? { cpu.push_str(s); }
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => cpu.push_str(&input.next_line()),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
