use crate::cursor::Cursor;
use crate::dir::Dir;
use itertools::repeat_n;
use std::collections::BTreeSet;

mod cursor;
mod dir;
mod point;

fn main() -> anyhow::Result<()> {
  let mut cursor = Cursor::default();
  let mut marks = BTreeSet::new();

  let input = std::fs::read_to_string("day9/input")?;

  let dirs = input
    .split('\n')
    .filter_map(|row| row.split_once(' '))
    .map(|(dir, count)| {
      (dir.parse::<Dir>().unwrap(), count.parse::<usize>().unwrap())
    })
    .flat_map(|(dir, count)| repeat_n(dir, count));

  for dir in dirs {
    cursor.go(dir);
    marks.insert(cursor.tail.clone());
  }

  println!("{:?}", marks.len());

  Ok(())
}
