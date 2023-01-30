use std::collections::HashMap;
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub enum Op {
  Pow2,
  Plus(usize),
  Mult(usize),
}

impl Op {
  pub fn compute(&self, worry_level: usize) -> usize {
    match self {
      Op::Pow2 => worry_level * worry_level,
      Op::Plus(x) => worry_level + x,
      Op::Mult(x) => worry_level * x,
    }
  }
}

#[derive(Debug)]
pub struct Monkey {
  id: usize,
  items: Vec<usize>,
  op: Op,
  branch: (usize, usize, usize),
}

impl Monkey {
  pub fn next_monkey(&self, worry_level: usize) -> usize {
    if worry_level % self.branch.0 == 0 {
      self.branch.1
    } else {
      self.branch.2
    }
  }
}

pub struct Simulator {
  pub monkeys: Vec<Monkey>,
  pub items: HashMap<usize, Vec<usize>>,
  pub inspections: Vec<usize>,
  pub module: usize,
}

impl Simulator {
  pub fn round(&mut self) {
    let Self {
      monkeys,
      items,
      inspections,
      module,
    } = self;

    for monkey in monkeys {
      let queue = items.insert(monkey.id, vec![]).unwrap();
      inspections[monkey.id] += queue.len();

      for it in queue {
        let it = monkey.op.compute(it) % *module;
        let id = monkey.next_monkey(it);
        items.get_mut(&id).unwrap().push(it);
      }
    }
  }
}

fn main() -> Result<()> {
  let f = std::fs::read_to_string("day11/input")?;

  let pattern = Regex::new(
    r#"Monkey (?P<id>\d+):\n {2}Starting items: (?P<items>.+)\n {2}Operation: new = old (?P<op>[+*]) (?P<arg>old|\d+)\n {2}Test: divisible by (?P<if>\d+)\n.+?(?P<then>\d+)\n.+?(?P<else>\d+)\n"#,
  )?;

  let notes = pattern
    .captures_iter(f.as_str())
    .map(|c| -> Option<Monkey> {
      Some(Monkey {
        id: c.name("id")?.as_str().parse().ok()?,
        items: c
          .name("items")?
          .as_str()
          .split(", ")
          .map(|n| n.parse().ok().unwrap())
          .collect_vec(),
        branch: (
          c.name("if")?.as_str().parse().ok()?,
          c.name("then")?.as_str().parse().ok()?,
          c.name("else")?.as_str().parse().ok()?,
        ),
        op: match (c.name("op")?.as_str(), c.name("arg")?.as_str()) {
          ("*", "old") => Op::Pow2,
          ("*", num) => Op::Mult(num.parse().ok()?),
          ("+", num) => Op::Plus(num.parse().ok()?),
          _ => return None,
        },
      })
    })
    .map(|it| it.unwrap())
    .collect_vec();

  let mut s = Simulator {
    module: notes.iter().map(|m| m.branch.0).product(),
    items: notes
      .iter()
      .map(|m| (m.id, m.items.clone()))
      .collect(),
    inspections: vec![0; notes.len()],
    monkeys: notes,
  };

  for _ in 0..10_000 {
    s.round();
  }

  s.inspections.sort_by_key(|&p| -(p as i64));
  println!("{:?}", s.inspections.iter().take(2).product::<usize>());

  Ok(())
}
