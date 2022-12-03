use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::BitAnd;
use itertools::{chain, Itertools};
use anyhow::Result;

fn main() -> Result<()> {
  let file = File::open("input")?;

  let x = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .map(|l| HashSet::<char>::from_iter(l.chars()))
    .tuples::<(_, _, _)>()
    .filter_map(|(r1, r2, r3)| {
      r1.bitand(&r2)
        .bitand(&r3)
        .into_iter()
        .next()
        .and_then(|ch| chain![[' '], 'a'..='z', 'A'..='Z'].position(|p| p == ch))
    })
    .sum::<usize>();

  println!("{x}");
  Ok(())
}
