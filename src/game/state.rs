use crate::GameState;
use std::{fmt::Display, rc::Rc};

use super::Connect4;

pub struct State {
  game: Rc<Connect4>,
  player: u64,
  mask: u64,
  moves: Vec<u8>
}

impl State {
  pub fn new(game: Rc<Connect4>, player: u64, mask: u64) -> Self {
    State {
      game,
      player, mask,
      moves: Vec::new()
    }
  }
}

impl GameState for State {
  fn can_play(&self, a: u16) -> bool {
    let flag: bool = (a as u8) < self.game.width;
    flag &&(self.mask & self.game.col_top_mask(a as u8) == 0)
  }

  fn play(&mut self, a: u16) -> Result<(), &'static str> {
    if !self.can_play(a) {
      return Err("Not a valid move");
    }
    let col = a as u8;
    self.moves.push(col);
    let move_mask = (self.mask + 
      self.game.col_bottom_mask(col)) & 
      self.game.col_mask(col);
    self.player ^= self.mask;
    self.mask |= move_mask;
    Ok(())
  }

  fn unplay(&mut self) -> Result<(), &'static str> {
      todo!()
  }

  fn key(&self) -> u64 {
      todo!()
  }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.game.width;
        let h = self.game.height;
        let mut stones_str: String = String::with_capacity((w * h + h) as usize + 1);
        stones_str.push('\n');
        let first_to_play: bool = self.moves.len() % 2 == 0;
        for row_i in 0..h {
          let h_offset = h - 1 - row_i;
          
          let mask = self.mask;
          for col_i in 0..w {
            let offset = h_offset + (h + 1) * col_i;
            let player = (self.player >> offset == 0) ^ first_to_play;
            let to_push = if mask >> offset & 1 == 1 {
              if player {
                'o'
              } else {
                'x'
              }
            } else {
              '.'
            };
            stones_str.push(to_push);
          }
          stones_str.push('\n');
        }
        
        write!(f, "{}", stones_str)
    }
}