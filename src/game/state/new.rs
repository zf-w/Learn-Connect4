use std::rc::Rc;

use crate::Connect4;

impl super::State {
  pub fn new(game: Rc<Connect4>, player: u64, mask: u64, moves_num: usize) -> Self {
    let len = game.total_stones() as usize;
    Self {
      game,
      player, mask,
      moves: Vec::with_capacity(len),
      moves_num
    }
  }
}