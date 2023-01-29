use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use anyhow::Result;
use crate::Gesture::{Paper, Rock, Scissors};
use crate::Outcome::{Won, Lost, Draw};

fn main() -> Result<()> {
  let file = File::open("input")?;
  let answer = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .map(|l| l.parse::<Round>().unwrap())
    .map(|round| round.score())
    .sum::<i64>();

  println!("{:?}", answer);
  Ok(())
}

pub struct Round {
  pub op: Gesture,
  pub me: Gesture,
  pub out: Outcome,
}

impl Round {
  pub fn new(op: Gesture, out: Outcome) -> Self {
    let me = match out {
      Draw => op,
      Won => op.beaten(),
      Lost => op.strikes(),
    };

    Round { op, me, out }
  }

  pub fn score(&self) -> i64 {
    // the score for the shape you selected plus the score for the outcome of the round
    self.me.score() + self.outcome().score()
  }

  pub fn outcome(&self) -> Outcome {
    match (self.op, self.me) {
      (x, y) if x == y => Draw,
      (x, y) if y.strikes() == x => Won,
      _ => Lost,
    }
  }
}

impl FromStr for Round {
  type Err = Infallible;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let (op, out) = s.split_once(' ').unwrap();
    Ok(Round::new(op.parse()?, out.parse()?))
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gesture {
  Rock,
  Paper,
  Scissors,
}

impl Gesture {
  pub fn score(&self) -> i64 {
    match self {
      Rock => 1,
      Paper => 2,
      Scissors => 3,
    }
  }

  pub fn strikes(&self) -> Gesture {
    match self {
      Rock => Scissors,
      Paper => Rock,
      Scissors => Paper,
    }
  }

  pub fn beaten(&self) -> Gesture {
    self.strikes().strikes()
  }
}

impl FromStr for Gesture {
  type Err = Infallible;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Ok(match s {
      "A" | "X" => Rock,
      "B" | "Y" => Paper,
      "C" | "Z" => Scissors,
      _ => panic!(),
    })
  }
}

#[derive(Clone, Copy)]
pub enum Outcome {
  Won,
  Draw,
  Lost,
}

impl Outcome {
  pub fn score(&self) -> i64 {
    match self {
      Won => 6,
      Draw => 3,
      Lost => 0,
    }
  }
}

impl FromStr for Outcome {
  type Err = Infallible;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Ok(match s {
      "X" => Lost,
      "Y" => Draw,
      "Z" => Won,
      _ => panic!(),
    })
  }
}
