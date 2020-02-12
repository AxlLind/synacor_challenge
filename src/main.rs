use std::fs;
use std::io::Result;
use itertools::Itertools;

mod cpu;
use cpu::CPU;

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

fn main() -> Result<()> {
  let program = read_program("./challenge.bin")?;
  let mut cpu = CPU::new(&program);
  cpu.execute();
  Ok(())
}
