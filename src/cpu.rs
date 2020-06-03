use std::collections::VecDeque;

const HLT: usize = 0; const JT:  usize =  7; const NOT: usize = 14;
const SET: usize = 1; const JF:  usize =  8; const RD:  usize = 15;
const PSH: usize = 2; const ADD: usize =  9; const WRT: usize = 16;
const POP: usize = 3; const MUL: usize = 10; const CLL: usize = 17;
const EQ:  usize = 4; const MOD: usize = 11; const RET: usize = 18;
const GT:  usize = 5; const AND: usize = 12; const OUT: usize = 19;
const JMP: usize = 6; const OR:  usize = 13; const IN:  usize = 20;
const INST_LEN: [u16;22] = [0,3,2,2,4,4,0,3,3,4,4,4,4,4,3,3,3,2,0,2,0,1];

pub enum ExitCode { Output(u16), NeedInput, Halted }

pub struct CPU {
  pub reg: [u16; 8],
  stack: Vec<u16>,
  input: VecDeque<u8>,
  pc: u16,
  mem: [u16; 0x8000],
}

impl CPU {
  pub fn new(program: &[u16]) -> Self {
    let mut mem = [0; 0x8000];
    mem[..program.len()].clone_from_slice(program);
    Self {
      reg: [0; 8],
      stack: Vec::new(),
      input: VecDeque::new(),
      pc: 0,
      mem,
    }
  }

  pub fn execute(&mut self) -> ExitCode {
    loop {
      let (op,a,b,c) = self.fetch_args();
      self.pc += INST_LEN.get(op).expect("Invalid opcode");
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
            self.reg[a] = i as u16;
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
    self.input.extend(s.bytes());
    self.input.push_back(b'\n');
  }

  fn fetch_args(&self) -> (usize, usize, u16, u16) {
    let op = self.mem[self.pc as usize] as usize;
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
