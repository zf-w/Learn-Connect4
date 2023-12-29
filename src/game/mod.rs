use std::rc;

mod state;
mod masks;

pub use state::State;
pub struct Connect4 {
  width: u8,
  height: u8,
  masks: Vec<u64>,
  me: rc::Weak<Self>
}

impl Connect4 {
  pub fn new(width: u8, height: u8) -> rc::Rc<Self> {
    let mut masks: Vec<u64> = Vec::with_capacity((width * 3 + 2) as usize);
    masks.push(masks::board_bottom_mask(width, height));
    masks.push(masks::board_mask(width, height));
    std::rc::Rc::new_cyclic(|me| -> Self {
      Connect4 {
        width, height, masks,
        me: me.clone()
      }
    })
  }

  pub fn start(&self) -> state::State {
    state::State::new(
      rc::Rc::clone(&self.me.upgrade().unwrap()),
      0, 0)
  }


  pub fn mask_col_top(&self, col: u8) -> u64 {
    1 << ((self.height - 1) + col * (self.height + 1))
  }

  pub fn mask_col_bottom(&self, col: u8) -> u64 {
    1 << col * (self.height + 1)
  }

  pub fn mask_col(&self, col: u8) -> u64 {
    ((1 << self.height) - 1) << col * (self.height + 1)
  }

  pub fn mask_bottom(&self) -> &u64 {
    self.masks.get(0).expect("bottom mask")
  }

  pub fn mask_full(&self) -> &u64 {
    self.masks.get(1).expect("Full board mask")
  }
}

#[cfg(test)]
mod test;