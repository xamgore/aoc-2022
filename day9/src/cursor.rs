use std::fmt::Debug;

use crate::dir::Dir;
use crate::point::Point;

#[derive(Default)]
pub struct Cursor {
  body: [Point; 10],
}

impl Cursor {
  pub fn tail(&self) -> Point {
    self.body[9].clone()
  }

  pub fn go(&mut self, dir: Dir) {
    self.body[0].go(dir);

    for idx in 1..10 {
      let (l, r) = self.body.split_at_mut(idx);
      let head = l.last().unwrap();
      let tail = r.first_mut().unwrap();
      tail.follow(head);
    }
  }
}
