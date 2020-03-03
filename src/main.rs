use std::fs;
use std::io::{stdout, Result, Write};
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

fn write_buf(buf: &mut Vec<u8>) -> Result<()> {
  stdout().write_all(&buf)?;
  stdout().flush()?;
  buf.clear();
  Ok(())
}

fn manual_play(cpu: &mut CPU) -> Result<()> {
  let mut input = InputReader::new();
  let mut buf = Vec::new();
  loop {
    match cpu.execute() {
      ExitCode::Output(i) => buf.push(i as u8),
      ExitCode::NeedInput => {
        write_buf(&mut buf)?;
        cpu.push_str(&input.next_line());
      },
      ExitCode::Halted => break,
    }
  }
  write_buf(&mut buf)
}

fn finish_challenge(cpu: &mut CPU) -> Result<()> {
  let inputs = fs::read_to_string("files/inputs.txt")?;
  cpu.push_str(&inputs);

  let mut buf = Vec::new();
  while let ExitCode::Output(i) = cpu.execute() {
    buf.push(i as u8);
  }
  write_buf(&mut buf)
}

fn main() -> Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  let mode = args.get(1).unwrap_or(&String::new()).clone();
  let program = read_program("files/challenge.bin")?;
  let mut cpu = CPU::new(&program);

  cpu.reg[7] = 25734; // computed in teleporter_setting.rs
  match &mode[..] {
    "manual" => manual_play(&mut cpu)?,
    _ => finish_challenge(&mut cpu)?,
  }
  Ok(())
}
