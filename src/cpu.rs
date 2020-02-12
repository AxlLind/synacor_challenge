use std::collections::VecDeque;
use std::io::{stdin, Read};

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

pub struct Cpu {
  pub reg: [u16;8],
  pub stack: VecDeque<u16>,
  pub mem: [u16; 0x8000],
  pub pc: usize,
}

impl Cpu {
  pub fn new(program: &[u16]) -> Self {
    let mut mem = [0; 0x8000];
    for i in 0..program.len() { mem[i] = program[i]; }
    Self {
      reg: [0;8],
      stack: VecDeque::new(),
      mem,
      pc: 0,
    }
  }

  pub fn execute(&mut self) {
    loop {
      let (op,a,b,c) = self.fetch_args();
      match op {
        HLT => return,
        JMP => self.pc = a,
        JT  => if a != 0 { self.pc = b as usize },
        JF  => if a == 0 { self.pc = b as usize },
        PSH => self.stack.push_back(a as u16),
        POP => self.reg[a] = self.stack.pop_back().expect("stack underflow"),
        SET => self.reg[a] = b,
        EQ  => self.reg[a] = (b == c) as u16,
        GT  => self.reg[a] = (b > c)  as u16,
        ADD => self.reg[a] = (b + c) % 0x8000,
        MUL => self.reg[a] = (b * c) % 0x8000,
        MOD => self.reg[a] = (b % c) % 0x8000,
        AND => self.reg[a] = (b & c) % 0x8000,
        OR  => self.reg[a] = (b | c) % 0x8000,
        NOT => self.reg[a] = !b      % 0x8000,
        RD  => self.reg[a] = self.mem[b as usize],
        WRT => self.mem[a] = b,
        CLL => {
          self.stack.push_back(self.pc as u16);
          self.pc = a;
        },
        RET => match self.stack.pop_back() {
          Some(a) => self.pc = a as usize,
          None    => return,
        },
        OUT => print!("{}", a as u8 as char),
        IN  => self.reg[a] = self.read_char(),
        NOP => {}
        _   => unreachable!("invalid opcode {}", op)
      }
    }
  }

  fn read_char(&self) -> u16 {
    stdin().lock().bytes().next().unwrap().unwrap() as u16
  }

  fn fetch_args(&mut self) -> (u16,usize,u16,u16) {
    let op = self.mem[self.pc];
    let a = match op {
      PSH|JMP|JT|JF|CLL|WRT => self.read_adr(1),
      _ => self.mem[self.pc+1] - 0x8000,
    };
    let b = self.read_adr(2);
    let c = self.read_adr(3);

    self.pc += match op {
      HLT|RET|JMP              => 0,
      NOP                      => 1,
      PSH|POP|CLL|OUT|IN       => 2,
      SET|JT|JF|NOT|RD|WRT     => 3,
      EQ|GT|ADD|MUL|MOD|AND|OR => 4,
      _ => unreachable!("invalid op {}", op),
    };

    (op, a as usize, b, c)
  }

  fn read_adr(&self, offset: usize) -> u16 {
    let v = self.mem[self.pc + offset];
    match v {
      0..=0x7FFF      => v,
      0x8000..=0x8007 => self.reg[(v - 0x8000) as usize],
      _               => unreachable!()
    }
  }
}
