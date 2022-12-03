use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;
use itertools::chain;
use anyhow::Result;

fn main() -> Result<()> {
  let file = File::open("input")?;

  let x = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .filter_map(|line| {
      let (left, right) = line.split_at(line.len() / 2);
      let set: HashSet<char> = HashSet::from_iter(left.chars());
      right.chars().skip_while(|ch| !set.contains(ch)).next()
    })
    .filter_map(|ch| chain![[' '], 'a'..='z', 'A'..='Z'].position(|p| p == ch))
    .sum::<usize>();

  println!("{x}");
  Ok(())
}
