use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
  let c = fs::read_to_string("input")?
    .chars()
    .tuple_windows::<(_, _, _, _)>()
    .position(|(a, b, c, d)| {
      let mut s = HashSet::with_capacity(4);
      s.insert(a) && s.insert(b) && s.insert(c) && s.insert(d)
    });

  println!("{}", c.unwrap() + 4);

  Ok(())
}
