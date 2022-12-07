use itertools::Itertools;
use std::collections::HashMap;

pub type FsId = usize;

#[derive(Debug, Eq, PartialEq)]
pub enum FsEntityKind {
  Dir,
  File,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FsEntity {
  pub name: String,
  pub size: usize,
  pub kind: FsEntityKind,
  pub children: HashMap<String, FsId>,
}

impl FsEntity {
  pub fn dir(name: &str) -> FsEntity {
    FsEntity {
      name: name.into(),
      size: 0,
      kind: FsEntityKind::Dir,
      children: Default::default(),
    }
  }

  pub fn file(name: &str, size: usize) -> FsEntity {
    FsEntity {
      name: name.into(),
      size,
      kind: FsEntityKind::File,
      children: Default::default(),
    }
  }
}

#[derive(Debug)]
pub struct FileSystem {
  pub store: Vec<FsEntity>,
}

impl FileSystem {
  pub fn new() -> Self {
    FileSystem {
      store: vec![FsEntity::dir("/")],
    }
  }

  pub fn create_missing(&mut self, folder: FsId, child: FsEntity) -> usize {
    if let Some(&id) = self.store[folder].children.get(&child.name) {
      id
    } else {
      let id = self.store.len();
      self.store[folder].children.insert(child.name.clone(), id);
      self.store.push(child);
      id
    }
  }

  pub fn compute_sizes(&mut self, id: FsId) -> usize {
    if self.store[id].kind != FsEntityKind::Dir {
      return self.store[id].size;
    }

    let mut size = 0;
    for ch_id in self.store[id].children.values().cloned().collect_vec() {
      size += self.compute_sizes(ch_id);
    }

    self.store[id].size = size;
    size
  }
}
