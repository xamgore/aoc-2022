use itertools::{unfold, Itertools};

use crate::opcode::OpCode;
use crate::runtime::Runtime;

mod opcode;
mod runtime;

fn main() -> anyhow::Result<()> {
  let f = std::fs::read_to_string("day10/input")?;

  let mut runtime = f
    .trim()
    .split('\n')
    .filter_map(|row| row.parse::<OpCode>().ok())
    .collect::<Runtime>();

  let s = unfold(runtime, |r| r.step())
    .map(|(cycle, x)| {
      let idx = (cycle - 1) % 40;
      if x - 1 <= idx && idx <= x + 1 {
        "#"
      } else {
        "."
      }
    })
    .take(40 * 6)
    .chunks(40)
    .into_iter()
    .map(String::from_iter)
    .join("\n");

  println!("screen:\n{s}");

  Ok(())
}
