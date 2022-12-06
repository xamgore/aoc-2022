use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let s = fs::read_to_string("input")?;

  let c = (0..s.len())
    .map(|idx| &s[idx..(idx + 14)])
    .position(|sl| {
      let mut set = HashSet::with_capacity(14);
      sl.chars().fold(true, |acc, ch| (acc && set.insert(ch)))
    });

  println!("{}", c.unwrap() + 14);

  Ok(())
}
