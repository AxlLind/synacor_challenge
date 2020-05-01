use std::collections::VecDeque;

const HLT: u16 = 0;  const SET: u16 = 1;  const PSH: u16 = 2;
const POP: u16 = 3;  const EQ:  u16 = 4;  const GT:  u16 = 5;
const JMP: u16 = 6;  const JT:  u16 = 7;  const JF:  u16 = 8;
const ADD: u16 = 9;  const MUL: u16 = 10; const MOD: u16 = 11;
const AND: u16 = 12; const OR:  u16 = 13; const NOT: u16 = 14;
const RD:  u16 = 15; const WRT: u16 = 16; const CLL: u16 = 17;
const RET: u16 = 18; const OUT: u16 = 19; const IN:  u16 = 20;
const INST_LEN: [u16;22] = [0,3,2,2,4,4,0,3,3,4,4,4,4,4,3,3,3,2,0,2,0,1];

pub enum ExitCode { Output(u16), NeedInput, Halted }

pub struct CPU {
  pub reg: [u16; 8],
  mem: [u16; 0x8000],
  stack: Vec<u16>,
  input: VecDeque<u16>,
  pc: u16,
}

impl CPU {
  pub fn new(program: &[u16]) -> Self {
    let mut mem = [0; 0x8000];
    mem[..program.len()].clone_from_slice(program);
    Self {
      reg: [0; 8],
      mem,
      stack: Vec::new(),
      input: VecDeque::new(),
      pc: 0,
    }
  }

  pub fn execute(&mut self) -> ExitCode {
    loop {
      let (op,a,b,c) = self.fetch_args();
      self.pc += INST_LEN[op as usize];
      match op {
        SET => self.reg[a] = b,
        OR  => self.reg[a] = b | c,
        AND => self.reg[a] = b & c,
        MOD => self.reg[a] = b % c,
        NOT => self.reg[a] = !b & 0x7FFF,
        ADD => self.reg[a] = (b + c) & 0x7FFF,
        MUL => self.reg[a] = (b * c) & 0x7FFF,
        GT  => self.reg[a] = (b > c) as u16,
        EQ  => self.reg[a] = (b == c) as u16,
        POP => self.reg[a] = self.stack.pop().unwrap(),
        RD  => self.reg[a] = self.mem[b as usize],
        WRT => self.mem[a] = b,
        PSH => self.stack.push(a as u16),
        RET => self.pc = self.stack.pop().unwrap(),
        JMP => self.pc = a as u16,
        JT  => if a != 0 { self.pc = b },
        JF  => if a == 0 { self.pc = b },
        HLT => return ExitCode::Halted,
        OUT => return ExitCode::Output(a as u16),
        IN  => match self.input.pop_front() {
          Some(i) => {
            self.reg[a] = i;
            self.pc += 2;
          }
          None => return ExitCode::NeedInput,
        },
        CLL => {
          self.stack.push(self.pc);
          self.pc = a as u16;
        },
        _ => {},
      }
    }
  }

  pub fn push_str(&mut self, s: &str) {
    self.input.extend(s.bytes().map(|b| b as u16));
    self.input.push_back(b'\n' as u16);
  }

  fn fetch_args(&self) -> (u16,usize,u16,u16) {
    let op = self.mem[self.pc as usize];
    let a = match op {
      PSH|JMP|JT|JF|CLL|WRT|OUT => self.read_adr(1),
      _ => self.mem[self.pc as usize + 1] - 0x8000,
    };
    (op, a as usize, self.read_adr(2), self.read_adr(3))
  }

  fn read_adr(&self, offset: usize) -> u16 {
    let v = self.mem[self.pc as usize + offset];
    *self.reg.get((v - 0x8000) as usize).unwrap_or(&v)
  }
}
