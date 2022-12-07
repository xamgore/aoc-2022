use crate::fs::{FileSystem, FsEntity, FsId};

#[derive(Debug)]
pub struct Terminal {
  pub fs: FileSystem,
  pub path: Vec<FsId>,
}

impl Terminal {
  pub fn new() -> Self {
    Self {
      path: vec![0],
      fs: FileSystem::new(),
    }
  }

  pub fn cwd(&self) -> FsId {
    self.path.last().cloned().unwrap_or_default()
  }

  pub fn touch(&mut self, entity: FsEntity) -> FsId {
    self.fs.create_missing(self.cwd(), entity)
  }

  pub fn cd(mut self, to: &str) -> Self {
    match to {
      ".." => {
        self.path.pop();
      }
      "/" => {
        self.path.clear();
      }
      _ => {
        let id = self.touch(FsEntity::dir(to));
        self.path.push(id);
      }
    }
    self
  }
}
