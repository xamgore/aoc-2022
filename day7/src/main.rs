mod fs;
mod log;
mod terminal;

use crate::fs::FsEntityKind;
use crate::log::LogItem;
use crate::terminal::Terminal;
use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<()> {
  let file = File::open("input")?;

  let mut terminal = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .filter_map(|l| l.parse::<LogItem>().ok())
    .fold(Terminal::new(), |mut term, it| match it {
      LogItem::Cd(to) => term.cd(&to),
      LogItem::Ls => term,
      LogItem::LsItem(it) => {
        term.touch(it);
        term
      }
    });

  terminal.fs.compute_sizes(0);

  let cumulative_size: usize = terminal
    .fs
    .store
    .iter()
    .filter(|it| it.kind == FsEntityKind::Dir)
    .map(|it| it.size)
    .filter(|&size| size <= 100_000)
    .sum();

  println!("{cumulative_size}");

  Ok(())
}
