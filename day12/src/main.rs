use std::collections::{BTreeSet, VecDeque};

fn main() {
  let mut grid = Grid::new(include_str!("../input"));
  let mut queue = VecDeque::from([(grid.start, 0)]);

  while let Some((pos, steps)) = queue.pop_front() {
    if pos == grid.end {
      println!("steps: {steps}");
      break;
    }

    if !grid.visited.insert(pos) {
      continue;
    }

    let level = grid.level_of(pos);

    queue.extend(
      grid
        .to2d(pos)
        .all_directions()
        .into_iter()
        .filter(|p| grid.in_bounds(p))
        .map(|p| grid.from2d(p))
        .filter(|p| level + 1 >= grid.level_of(*p))
        .map(|p| (p, steps + 1)),
    );
  }

  println!("done");
}

pub type Pos = i64;

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn all_directions(self) -> [Point; 4] {
    let Point { x, y } = self;
    [
      Point { x: x + 1, y },
      Point { x: x - 1, y },
      Point { x, y: y + 1 },
      Point { x, y: y - 1 },
    ]
  }
}

pub struct Grid<'a> {
  pub visited: BTreeSet<Pos>,
  pub map: &'a str,
  pub w: i64,
  pub h: i64,
  pub start: Pos,
  pub end: Pos,
}

impl<'a> Grid<'a> {
  pub fn new(map: &'a str) -> Grid<'a> {
    Self {
      map,
      visited: Default::default(),
      h: map.split("\n").count() as i64 - 1,
      w: map.split("\n").next().unwrap().len() as i64,
      start: map.find('S').unwrap() as Pos,
      end: map.find('E').unwrap() as Pos,
    }
  }

  pub fn level_of(&self, pos: Pos) -> u8 {
    lvl(self.map.chars().nth(pos as usize).unwrap())
  }

  pub fn from2d(&self, point: Point) -> Pos {
    point.y * (self.w + 1) + point.x
  }

  pub fn to2d(&self, pos: Pos) -> Point {
    let x = pos % (self.w + 1);
    let y = pos / (self.w + 1);
    Point { x, y }
  }

  pub fn in_bounds(&self, point: &Point) -> bool {
    0 <= point.x && point.x < self.w && 0 <= point.y && point.y < self.h
  }
}

pub fn lvl(x: char) -> u8 {
  match x {
    'S' => lvl('a'),
    'E' => lvl('z'),
    'a'..='z' => x as u8,
    _ => panic!(),
  }
}
