use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
  let f = std::fs::read_to_string("day8/input")?;

  let m = f
    .trim()
    .split('\n')
    .map(|row| {
      row
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect_vec()
    })
    .collect_vec();

  let (width, height) = (m[0].len(), m.len());
  let mut acc = Vec::with_capacity(2 * width + 2 * height);

  for i in 0..height {
    acc.extend(visible((0..width).map(|j| (i, j, m[i][j]))));
    acc.extend(visible((0..width).rev().map(|j| (i, j, m[i][j]))));
  }
  for j in 0..width {
    acc.extend(visible((0..height).map(|i| (i, j, m[i][j]))));
    acc.extend(visible((0..height).rev().map(|i| (i, j, m[i][j]))));
  }

  acc.sort_unstable();

  let p = acc
    .into_iter()
    .group_by(|&(pos, _)| pos)
    .into_iter()
    .map(|(pos, it)| (it.map(|(_, d)| d).product::<usize>(), pos))
    .max()
    .unwrap();

  println!("{:?}", p.0);

  Ok(())
}

fn visible(
  map: impl Iterator<Item = (usize, usize, usize)>,
) -> impl Iterator<Item = ((usize, usize), usize)> {
  let mut ds = [0; 10];

  map.enumerate().map(move |(idx, (i, j, h))| {
    let d = idx - ds[h..].iter().max().unwrap();
    ds[h] = idx;
    ((i, j), d)
  })
}
