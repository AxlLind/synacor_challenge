use std::fs;
use itertools::Itertools;
use easy_io::InputReader;

mod cpu;
use cpu::{CPU, ExitCode};

mod solve_coins;
use solve_coins::brute_force_coins;

fn read_program(path: &str) -> Vec<u16> {
  fs::read(path).unwrap()
    .iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect()
}

fn fetch_inputs() -> Vec<String> {
  let fetch_coins = fs::read_to_string("./inputs/fetch_coins.txt").unwrap();
  let after_coins = fs::read_to_string("./inputs/after_coins.txt").unwrap();
  let coin_order  = brute_force_coins().join("\n") + "\n";
  vec![fetch_coins, coin_order, after_coins]
}

fn main() {
  let program = read_program("./challenge.bin");
  let mut cpu = CPU::new(&program);
  let mut input = InputReader::new();

  for s in &fetch_inputs() { cpu.push_str(s); }
  loop {
    match cpu.execute() {
      ExitCode::NeedInput => cpu.push_str(&input.next_line()),
      ExitCode::Halted    => break,
    }
  }
}
