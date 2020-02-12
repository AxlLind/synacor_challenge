use std::collections::VecDeque;

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

pub enum ExitCode { NeedInput, Halted }

pub struct CPU {
  reg: [u16; 8],
  mem: [u16; 0x8000],
  stack: VecDeque<u16>,
  input: VecDeque<u16>,
  pc: usize,
}

impl CPU {
  pub fn new(program: &[u16]) -> Self {
    let mut mem = [0; 0x8000];
    mem[..program.len()].clone_from_slice(program);
    Self {
      reg: [0; 8],
      mem,
      stack: VecDeque::new(),
      input: VecDeque::new(),
      pc: 0,
    }
  }

  pub fn execute(&mut self) -> ExitCode {
    loop {
      let (op,a,b,c) = self.fetch_args();
      self.incr_pc(op);
      match op {
        HLT => return ExitCode::Halted,
        JMP => self.pc = a,
        JT  => if a != 0 { self.pc = b as usize },
        JF  => if a == 0 { self.pc = b as usize },
        PSH => self.stack.push_back(a as u16),
        POP => self.reg[a] = self.stack.pop_back().expect("stack underflow"),
        SET => self.reg[a] = b,
        EQ  => self.reg[a] = (b == c) as u16,
        GT  => self.reg[a] = (b > c)  as u16,
        ADD => self.reg[a] = (b + c) & 0x7FFF,
        MUL => self.reg[a] = (b * c) & 0x7FFF,
        NOT => self.reg[a] = !b & 0x7FFF,
        MOD => self.reg[a] = b % c,
        AND => self.reg[a] = b & c,
        OR  => self.reg[a] = b | c,
        RD  => self.reg[a] = self.mem[b as usize],
        WRT => self.mem[a] = b,
        OUT => print!("{}", a as u8 as char),
        IN  => match self.input.pop_front() {
          Some(i) => {
            self.reg[a] = i;
            self.pc += 2;
          }
          None => return ExitCode::NeedInput,
        },
        CLL => {
          self.stack.push_back(self.pc as u16);
          self.pc = a;
        },
        RET => match self.stack.pop_back() {
          Some(a) => self.pc = a as usize,
          None    => return ExitCode::Halted,
        },
        NOP => {},
        _   => panic!("invalid opcode {}", op)
      }
    }
  }

  pub fn push_input<T: Into<u16>>(&mut self, t: T) {
    self.input.push_back(t.into());
  }
}

// private methods
impl CPU {
  fn fetch_args(&self) -> (u16,usize,u16,u16) {
    let op = self.mem[self.pc];
    let a = match op {
      PSH|JMP|JT|JF|CLL|WRT|OUT => self.read_adr(1),
      _ => self.mem[self.pc+1] - 0x8000,
    };
    let b = self.read_adr(2);
    let c = self.read_adr(3);
    (op, a as usize, b, c)
  }

  fn incr_pc(&mut self, op: u16) {
    self.pc += match op {
      EQ|GT|ADD|MUL|MOD|AND|OR => 4,
      SET|JT|JF|NOT|RD|WRT     => 3,
      PSH|POP|CLL|OUT          => 2,
      NOP                      => 1,
      _                        => 0,
    };
  }

  fn read_adr(&self, offset: usize) -> u16 {
    let v = self.mem[self.pc + offset];
    match v {
      0x0000..=0x7FFF => v,
      0x8000..=0x8007 => self.reg[(v - 0x8000) as usize],
      _               => unreachable!()
    }
  }
}
