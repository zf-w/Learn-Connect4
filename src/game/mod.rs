use std::rc;

mod state;
mod masks;
pub use state::_format_board;
pub use state::State;
pub use state::bound::StateResult;
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
      0, 0, 0)
  }

  pub fn width(&self) -> u8 {
    self.width
  }

  pub fn height(&self) -> u8 {
    self.height
  }

  pub fn total_stones(&self) -> u16 {
    (self.width * self.height) as u16
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

  pub fn mask_col_full(&self, col: u8) -> u64 {
    ((1 << self.height + 1) - 1) << col * (self.height + 1)
  }

  pub fn mask_bottom(&self) -> &u64 {
    self.masks.get(0).expect("bottom mask")
  }

  pub fn mask_full(&self) -> &u64 {
    self.masks.get(1).expect("Full board mask")
  }

  pub fn flip_c4_board(&self, p_mask: u64) -> u64 {
    let mut n_mask: u64 = 0;
    let w = self.width;
    let off = self.height + 1;
    for col in 0..w {
      let n_col = w - 1 - col;
      let curr_col = (p_mask & self.mask_col_full(col)) >> col as u16 * off as u16;
      n_mask |= curr_col << n_col as u16 * off as u16;
    }
    n_mask
  }

  pub fn count_mask_stones(&self, m: u64) -> usize {
    let w = self.width;
    let off = self.height + 1;
    let mut count = 0usize;
    for col in 0..w {
      let curr_off: u16 = col as u16 * off as u16;
      let curr_col = m & self.mask_col_full(col);
      if curr_col > 0 {
        count += (u64::ilog2(curr_col) - curr_off as u32) as usize;
      }
    }
    count
  }

  fn build_mask_and_player_from_combined(&self, m: u64) -> (u64, u64, usize) {
    let w = self.width;
    let off = self.height + 1;
    let mut count = 0usize;
    let mut mask: u64 = 0;
    for col in 0..w {
      let curr_off: u16 = col as u16 * off as u16;
      let curr_col = m & self.mask_col_full(col);
      let mut new_col_mask: u64 = 1;
      if curr_col > 0 {
        let col_count = (u64::ilog2(curr_col) - curr_off as u32) as usize;
        count += col_count;
        new_col_mask <<= col_count;
        new_col_mask -= 1;
      }
      mask |= new_col_mask << curr_off;
    }
    (mask, m - (mask + self.mask_bottom()), count)
  }

  pub fn start_from_mask(&self, m: u64) -> state::State {
    let (mask, player, moves_num) = self.build_mask_and_player_from_combined(m);
    
    state::State::new(self.me.clone().upgrade().unwrap(), player, mask, moves_num)
  }
}

#[cfg(test)]
mod test;