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

  pub fn reset(&mut self) {
    for v in self.t.iter_mut() {
      v.0 = 0;
      v.1 = 0;
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

#[cfg(test)]
mod test {
  use super::TranspositionTable;
  #[test]
  fn check_put_get() {
    let mut t = TranspositionTable::new(7);
    t.put(100, -1);
    assert_eq!(t.get(100), Some(-1));
  }
}