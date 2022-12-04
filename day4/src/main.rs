use itertools::Itertools;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
  let f = std::fs::File::open("input")?;
  let x = BufReader::new(f)
    .lines()
    .map(|l| l.unwrap())
    .filter_map(|l| {
      l.split(|c| c == ',' || c == '-')
        .map(|s| s.parse::<i64>().unwrap())
        .collect_tuple::<(_, _, _, _)>()
    })
    .filter(|&(a, b, c, d)| {
      (a <= c && d <= b) || (c <= a && b <= d)
    })
    .count();

  println!("{x}");

  Ok(())
}
