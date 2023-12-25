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
    flag &&(self.mask & self.game.mask_col_top(a as u8) == 0)
  }

  fn possible_moves(&self) -> Vec<u16> {
      let moves_mask = self.possible_moves_mask();
      let mut res: Vec<u16> = Vec::with_capacity(self.game.width as usize);
      for col in 0..self.game.width {
        if (self.game.mask_col(col) & moves_mask) > 0 {
          res.push(col as u16);
        }
      }
      res
  }

  fn play(&mut self, a: u16) -> Result<(), &'static str> {
    if !self.can_play(a) {
      return Err("Not a valid move");
    }
    let col = a as u8;
    self.moves.push(col);
    let move_mask = (self.mask + 
      self.game.mask_col_bottom(col)) & 
      self.game.mask_col(col);
    self.player ^= self.mask;
    self.mask |= move_mask;
    Ok(())
  }

  fn unplay(&mut self) -> Result<(), &'static str> {
    let col = match self.moves.pop() {
      Some(v) => Ok(v),
      None => Err("That's already the initial state")
    }?;

    let stone_mask =((self.mask & self.game.mask_col(col)) +self.game.mask_col_bottom(col)) >> 1;
    self.mask &= stone_mask;
    self.player &= stone_mask;
    Ok(())
  }

  fn key(&self) -> u64 {
      self.player + self.mask
  }
}

impl State {
  fn possible_moves_mask(&self) -> u64 {
    (self.mask + self.game.mask_bottom()) & self.game.mask_full()
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
            let player = ((self.player >> offset) & 1 == 0) ^ first_to_play;
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

#[cfg(test)]
mod test {
    use crate::{Connect4, GameState};

  #[test]
  fn check_possible_moves() {
    let game = Connect4::new(7, 6);
    let mut s = game.start();
    let got0 = s.possible_moves_mask();
    let expected0: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;
    assert_eq!(got0, expected0);
    assert!(s.play(2).is_ok());
    let got1 = s.possible_moves_mask();
    let expected1: u64 = 0b0000001_0000001_0000001_0000001_0000010_0000001_0000001;
    assert_eq!(got1, expected1);
  }
}