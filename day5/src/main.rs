use std::{fs, iter};
use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
  let content = fs::read_to_string("input")?;

  let (state, algo) = content.split_once("\n\n").unwrap();
  let state = state.chars().skip(1).step_by(4).collect_vec();

  let m = state.last().and_then(|ch| ch.to_digit(10)).unwrap() as usize;
  let n = state.len() / m - 1;

  let mut stacks: Vec<Vec<char>> =
    (0..m).map(|col|
      (0..n)
        .rev()
        .map(|row| state[(row * m) + col])
        .filter(|ch| *ch != ' ')
        .collect_vec()
    ).collect_vec();

  Regex::new(r#"move (\d+) from (\d+) to (\d+)"#)?
    .captures_iter(algo)
    .flat_map(|caps| caps
      .iter()
      .skip(1)
      .filter_map(|mat| mat.and_then(|mat| mat.as_str().parse().ok()))
      .collect_tuple::<(usize, usize, usize)>()
    )
    .enumerate()
    .for_each(|(idx, (amount, from, to))| {
      for _i in 0..amount {
        let x = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(x);
      }
    });

  for _i in 0..m {
    print!("{}", stacks[_i].pop().unwrap_or('_'));
  }

  Ok(())
}
