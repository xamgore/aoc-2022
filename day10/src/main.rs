use crate::opcode::OpCode;
use crate::runtime::Runtime;

mod opcode;
mod runtime;

fn main() -> anyhow::Result<()> {
  let f = std::fs::read_to_string("day10/input")?;

  let mut runtime =
    f.trim().split('\n').filter_map(|row| row.parse::<OpCode>().ok()).collect::<Runtime>();

  let mut sum = 0;
  while let Some((cycle, strength)) = runtime.step() {
    if (cycle == 20) || (cycle - 20) % 40 == 0 {
      sum += strength;
      println!("{cycle}th cycle: {strength}");
    }
  }

  println!("total: {sum}");

  Ok(())
}
