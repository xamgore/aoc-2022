use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use itertools::Itertools;
use anyhow::Result;

fn main() -> Result<()> {
  let file = File::open("input")?;
  let answer = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .group_by(|l| !l.is_empty())
    .into_iter()
    .filter(|(has_elements, _)| *has_elements)
    .map(|(_, group)| group.map(|it| it.parse::<i64>().unwrap()).sum::<i64>())
    .map(|it| -it)
    .k_smallest(3)
    .sum::<i64>();

  println!("{:?}", -answer);
  Ok(())
}
