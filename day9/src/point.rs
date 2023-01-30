use crate::dir::Dir;

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn go(&mut self, dir: Dir) {
    match dir {
      Dir::U => self.y += 1,
      Dir::D => self.y -= 1,
      Dir::R => self.x += 1,
      Dir::L => self.x -= 1,
    }
  }
}
