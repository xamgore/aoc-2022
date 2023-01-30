use crate::dir::Dir;
use crate::point::Point;
use std::cmp::max;

#[derive(Default, Debug)]
pub struct Cursor {
  pub head: Point,
  pub tail: Point,
}

impl Cursor {
  pub fn distance(&self) -> usize {
    max(
      (self.head.x - self.tail.x).abs(),
      (self.head.y - self.tail.y).abs(),
    ) as usize
  }

  pub fn go(&mut self, dir: Dir) {
    self.head.go(dir);

    if self.distance() >= 2 {
      self.tail = self.head.clone();
      self.tail.go(dir.opposite());
    }
  }
}
