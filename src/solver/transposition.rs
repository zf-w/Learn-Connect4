use std::i8;

pub(crate) struct TranspositionTable {
  t: Vec<(u64, i8)>
}

impl TranspositionTable {
  pub fn new(size: usize) -> Self {
    let t: Vec<(u64, i8)> = vec![(0, 0); size];
    Self {
      t
    }
  }

  fn index(&self, k: u64) -> usize {
    k as usize % self.t.len()
  }

  pub fn put(&mut self, k: u64, v: i8) {
    let i = self.index(k);
    self.t[i] = (k, v);
  }

  pub fn get(&self, k: u64) -> Option<i8> {
    let i = self.index(k);
    let stored_key = self.t[i].0;
    if stored_key == k {
      Some(self.t[i].1)
    } else {
      None
    }
  }
}