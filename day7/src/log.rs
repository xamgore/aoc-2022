use crate::fs::FsEntity;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum LogItem {
  Ls,
  LsItem(FsEntity),
  Cd(String),
}

impl FromStr for LogItem {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "$ ls" => Self::Ls,
      s if s.starts_with("$ cd") => Self::Cd(s[5..].to_string()),
      s if s.starts_with("dir") => Self::LsItem(FsEntity::dir(&s[4..])),
      _ => {
        let (size, name) = s.split_once(' ').unwrap();
        Self::LsItem(FsEntity::file(name, size.parse().unwrap()))
      }
    })
  }
}
