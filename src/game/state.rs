use crate::GameState;
use std::{rc::Rc, error::Error};

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

pub mod bound;

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

  fn play(&mut self, a: u16) -> Result<(), Box<dyn Error>> {
    if !self.can_play(a) {
      return Err("Not a valid move".into());
    }

    if self.is_winning_action(a) {
      return Err(format!("This is a winning action. Trying to play {a} in {}", &self).into());
    }

    let col = a as u8;
    self.moves.push(col);
    let move_mask = (self.possible_moves_mask()) & 
      self.game.mask_col(col);
    self.player ^= self.mask;
    self.mask |= move_mask;
    Ok(())
  }

  fn unplay(&mut self) -> Result<(), Box<dyn Error>> {
    let col = match self.moves.pop() {
      Some(v) => Ok(v),
      None => Err("That's already the initial state")
    }?;

    let stone_mask =((self.mask & self.game.mask_col(col)) +self.game.mask_col_bottom(col)) >> 1;
    let keep = !stone_mask;
    self.mask &= keep;
    self.player &= keep;
    self.player ^= self.mask;
    Ok(())
  }

  fn key(&self) -> u64 {
      let p_mask = self.player + (self.mask + self.game.mask_bottom());
      
      p_mask.max(self.game.flip_c4_board(p_mask))
  }
}

impl State {

  pub fn len(&self) -> usize {
    self.moves.len()
  }

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
    self.me_winning_mask() & self.possible_moves_mask() & self.game.mask_col(col) > 0
  }

  pub fn play_multiple(&mut self, moves: &Vec<u16>) -> Result<(), Box<dyn Error>> {
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
      
      let opponent_winning_next = opponent_winning & options;

      if opponent_winning_next > 0 && (opponent_winning_next & (opponent_winning_next - 1)) > 0 {
        return res;
      } else if opponent_winning_next > 0 {
        options &= opponent_winning_next;
      }

      options &= !(opponent_winning >> 1);

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

pub fn _format_board(b: u64, w: u8, h: u8) -> String {
  let mut res: String = String::with_capacity(64);
  for row_i in 0..=h {
    let h_offset = h - row_i;
    
    for col_i in 0..w {
      let offset = h_offset + (h + 1) * col_i;
      
      let to_push = if b >> offset & 1 == 1 {
        'o'
      } else {
        '.'
      };
      res.push(to_push);
    }
    res.push('\n');
  }
  res
}

mod display;

#[cfg(test)]
mod test;