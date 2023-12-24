use std::rc;

mod state;

pub use state::State;
pub struct Connect4 {
  width: u8,
  height: u8,
  me: rc::Weak<Self>
}

impl Connect4 {
  pub fn new(width: u8, height: u8) -> rc::Rc<Self> {
    std::rc::Rc::new_cyclic(|me| -> Self {
      Connect4 {
        width, height,
        me: me.clone()
      }
    })
  }

  pub fn start(&self) -> state::State {
    state::State::new(
      rc::Rc::clone(&self.me.upgrade().unwrap()),
      0, 0)
  }

  pub fn width(&self) -> u8 {
    self.width
  }

  pub fn height(&self) -> u8 {
    self.height
  }

  pub fn col_top_mask(&self, col: u8) -> u64 {
    1 << ((self.height - 1) + col * (self.height + 1))
  }

  pub fn col_bottom_mask(&self, col: u8) -> u64 {
    1 << col * (self.height + 1)
  }

  pub fn col_mask(&self, col: u8) -> u64 {
    ((1 << self.height) - 1) << col * (self.height + 1)
  }
}

#[cfg(test)]
mod test;