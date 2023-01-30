use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
  U,
  D,
  L,
  R,
  X,
}

impl Dir {
  pub fn opposite(&self) -> Self {
    match self {
      Dir::U => Dir::D,
      Dir::D => Dir::U,
      Dir::L => Dir::R,
      Dir::R => Dir::L,
      Dir::X => Dir::X,
    }
  }
}

impl FromStr for Dir {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "U" => Dir::U,
      "D" => Dir::D,
      "R" => Dir::R,
      "L" => Dir::L,
      _ => return Err(()),
    })
  }
}
