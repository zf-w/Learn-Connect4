use crate::GameState;
use std::rc::Rc;

use super::{Connect4, masks::get_winning_mask};

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
      let options = self.possible_moves_mask();
      
      let mut res: Vec<u16> = Vec::with_capacity(self.game.width as usize);
     
      for col in 0..self.game.width {
        if (self.game.mask_col(col) & options) > 0 {
          res.push(col as u16);
        }
      }
      res
  }

  fn play(&mut self, a: u16) -> Result<(), &'static str> {
    if !self.can_play(a) {
      return Err("Not a valid move");
    }

    if self.is_winning_action(a) {
      return Err("This is a winning action");
    }

    let col = a as u8;
    self.moves.push(col);
    let move_mask = (self.possible_moves_mask()) & 
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
    self.mask &= stone_mask ^ 0;
    self.player &= stone_mask ^ 0;
    self.mask ^= self.game.mask_full();
    Ok(())
  }

  fn key(&self) -> u64 {
      self.player + self.mask
  }

  fn bound(&self) -> (i32, i32) {
        todo!()
  }
}

impl State {
  fn possible_moves_mask(&self) -> u64 {
    (self.mask + self.game.mask_bottom()) & self.game.mask_full()
  }

  fn empty_space_mask(&self) -> u64 {
    self.game.mask_full() ^ self.mask
  }

  fn me_winning_mask(&self) -> u64 {
    get_winning_mask(&self.player, &self.game.height) & self.empty_space_mask()
  }

  fn opponent_winning_mask(&self) -> u64 {
    let opponent = &self.player ^ &self.mask;
    get_winning_mask(&opponent, &self.game.height) & self.empty_space_mask()
  }

  fn is_winning_action(&self, a: u16) -> bool {
    let col = a as u8;
    self.me_winning_mask() & self.game.mask_col(col) > 0
  }

  pub fn play_multiple(&mut self, moves: &Vec<u16>) -> Result<(), &'static str> {
    for a in moves.iter() {
      self.play(*a)?
    }
    Ok(())
  }

  fn heuristic_action_score(&self, a: u16) ->u16 {
    let col = a as u8;
    let move_mask = self.possible_moves_mask() & 
      self.game.mask_col(col);
    let stones = self.player | move_mask;
    let winning = get_winning_mask(&stones, &self.game.height) & self.empty_space_mask();
    fn bit_count(stones: &u64) -> u8 {
      let mut count: u8 = 0;
      let mut n = stones.clone();
      while n > 0 {
        n &= n - 1;
        count += 1;
      }
      count
    }
    let h_w = self.game.width as i16 / 2;
    let col_i16 = col as i16;
    let count = bit_count(&winning) as u16;

    count * (h_w + 1) as u16 + (h_w - (col_i16 - h_w).abs()) as u16
  }

  pub fn nonlosing_moves_sorted(&self) -> Vec<u16> {
      let len: usize = self.game.width as usize;
      let mut res: Vec<u16> = Vec::with_capacity(len);
      let mut scores: Vec<u16> = Vec::with_capacity(len);
      
      let mut options = self.possible_moves_mask();
      
      let opponent_winning = self.opponent_winning_mask();
      
      if opponent_winning > 0 && (opponent_winning & (opponent_winning - 1)) > 0 {
        return res;
      } else if opponent_winning > 0 {
        options &= opponent_winning;
      }

      let mut insert = |col: u16, score: u16| {
        let mut i = res.len();
        scores.push(0);
        res.push(0);
        while i > 0 && scores[i - 1] < score {
          scores[i] = scores[i - 1];
          res[i] = res[i - 1];
          i -= 1;
        }
        scores[i] = score;
        res[i] = col;
      };
      
      for col in 0..self.game.width as u16 {
        if (self.game.mask_col(col as u8) & options) > 0 {
          insert(col, self.heuristic_action_score(col));
        }
      }
      res
  }
}

mod display;

#[cfg(test)]
mod test;