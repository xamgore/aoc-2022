use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

use crate::opcode::OpCode;

struct State {
  pub cycle: i64,
  pub register: i64,
  pub pc: usize,
}

impl State {
  pub fn exec(&mut self, opcode: OpCode) {
    match opcode {
      OpCode::Noop => (),
      OpCode::Add { x } => self.register += x,
    }
  }
}

pub struct Runtime {
  state: State,
  source: Vec<OpCode>,
  waiting: Vec<(OpCode, i64)>,
}

impl Runtime {
  pub fn step(&mut self) -> Option<(i64, i64)> {
    let Runtime { waiting, state, source } = self;

    if state.pc >= source.len() && waiting.is_empty() {
      return None;
    }

    state.cycle += 1;

    if waiting.len() > 0 {
      *waiting = waiting
        .iter()
        .cloned()
        .filter(|&(op, till)| {
          if till == state.cycle {
            state.exec(op);
            false
          } else {
            true
          }
        })
        .collect_vec();
    }

    if waiting.is_empty() && state.pc < source.len() {
      let op = source[state.pc];
      state.pc += 1;

      waiting.push((op, state.cycle + op.duration()));
    }

    Some((state.cycle, state.register))
  }
}

impl FromIterator<OpCode> for Runtime {
  fn from_iter<T: IntoIterator<Item = OpCode>>(iter: T) -> Self {
    Runtime {
      state: State {
        pc: 0,
        cycle: 0,
        register: 1,
      },
      source: Vec::from_iter(iter),
      waiting: vec![(OpCode::Noop, 1)],
    }
  }
}

impl Display for Runtime {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(
      f,
      "R[cycle: {}, x: {}, pc: {}, wait: {:?}]",
      self.state.cycle, self.state.register, self.state.pc, self.waiting,
    )
  }
}
