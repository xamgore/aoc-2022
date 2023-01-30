use std::num::ParseIntError;
use std::str::FromStr;

use crate::opcode::OpCode::{Add, Noop};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
  Noop,
  Add { x: i64 },
}

impl OpCode {
  pub fn duration(&self) -> i64 {
    match self {
      Noop => 1,
      Add { .. } => 2,
    }
  }
}

impl FromStr for OpCode {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s == "noop" {
      Noop
    } else {
      let (_, r) = s.split_at("addx ".len());
      Add {
        x: r.parse::<i64>()?,
      }
    })
  }
}
