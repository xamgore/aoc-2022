use Dir::{D, L, R, U, X};

use crate::dir::Dir;

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn go(&mut self, dir: Dir) {
    match dir {
      U => self.y += 1,
      D => self.y -= 1,
      R => self.x += 1,
      L => self.x -= 1,
      X => (),
    }
  }

  pub fn follow(&mut self, other: &Point) {
    let (dx, dy) = (other.x - self.x, other.y - self.y);

    if dx.abs() == 2 || dy.abs() == 2 {
      let x = match dx {
        1 | 2 => R,
        -1 | -2 => L,
        _ => X,
      };

      let y = match dy {
        1 | 2 => U,
        -1 | -2 => D,
        _ => X,
      };

      self.go(x);
      self.go(y);
    }
  }
}
