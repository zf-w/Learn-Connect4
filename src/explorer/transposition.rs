use std::{i8, collections::HashMap};

pub struct TranspositionTable {
  t: Vec<Option<(u64, i8)>>,
  book: HashMap<u64, i8>
}

impl TranspositionTable {
  pub fn new(size: usize) -> Self {
    let t: Vec<Option<(u64, i8)>> = vec![None; size];
    Self {
      t,
      book: HashMap::with_capacity(1<<22)
    }
  }

  pub fn reset(&mut self) {
    for v in self.t.iter_mut() {
      *v = None;
    }
  }

  fn index(&self, k: u64) -> usize {
    k as usize % self.t.len()
  }

  pub fn put(&mut self, k: u64, v: i8, save: bool) {
    let i = self.index(k);
    let curr: &mut Option<(u64, i8)> = &mut self.t[i];
    if let Some((k0, v0)) = curr {
      if save {
        self.book.insert(*k0, *v0);
      }
      *k0 = k;
      *v0 = v;
    } else {
      *curr = Some((k, v));
    }
  }

  pub fn get(&self, k: u64) -> Option<i8> {
    let i = self.index(k);
    let curr = self.t[i];
    
    if let Some((stored_k, v)) = curr {
      if stored_k == k {
        Some(v)
      } else {
        None
      }
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
    t.put(100, -1, false);
    assert_eq!(t.get(100), Some(-1));
  }
}