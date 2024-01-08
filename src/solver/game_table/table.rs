use super::NegamaxResult;

pub(super) struct Table {
  t: Vec<Option<(u64, NegamaxResult)>>
}

impl Table {
  pub fn new(size: usize) -> Self {
    let t: Vec<Option<(u64, NegamaxResult)>> = vec![None; size];
    Self {
      t
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

  pub fn put(&mut self, k: u64, v: NegamaxResult) {
    let i = self.index(k);
    self.t[i] = Some((k, v));
  }

  pub fn get(&self, k: u64) -> Option<&NegamaxResult> {
    let i = self.index(k);
    let curr = &self.t[i];
    
    if let Some((k0, v0)) = curr {
      if *k0 == k {
        Some(v0)
      } else {
        None
      }
    } else {
      None
    }
  }
}