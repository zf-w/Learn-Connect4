use std::{rc::Rc, error::Error};

use crate::{Connect4, State, GameState};

use self::transposition::TranspositionTable;

mod transposition;

pub struct Solver {
  t: TranspositionTable,
  count: usize,
  game: Rc<Connect4>
}

impl Solver {
  pub fn new(game: Rc<Connect4>) -> Self {
    Self {
      t: TranspositionTable::new(8388593),
      count: 0,
      game
    }
  }

  pub fn count(&self) -> usize {
    self.count
  }

  fn negamax(&mut self, s: &mut State, mut bound: (i32, i32)) -> Result<i32, Box<dyn Error>> {
    // println!("Starting {}", s);
    self.count += 1;
    let new_bound = s.bound();
    if new_bound.0 == new_bound.1 {
      // println!("Finished {} score: {}", s, new_bound.0);
      return Ok(new_bound.0);
    }
    if bound.0 < new_bound.0 {
      bound.0 = new_bound.0;
      if bound.0 >= bound.1 {
        return Ok(bound.0);
      }
    }
    let curr_key = s.key();
    if let Some(v) = self.t.get(curr_key) {
      bound.1 = v as i32;
    }
    if bound.1 > new_bound.1 {
      bound.1 = new_bound.1;
      if bound.0 >= bound.1 {
        return Ok(bound.1);
      }
    }
    // println!("Looping with [{}, {}]", bound.0, bound.1);
    let actions = s.nonlosing_moves_sorted();
    for a in actions.iter() {
      s.play(*a)?;
      let score = -self.negamax(s, (-bound.1, -bound.0))?;
      s.unplay()?;
      if score > bound.0 {
        bound.0 = score;
      }
      if score >= bound.1 {
        return Ok(score);
      }
    }
    // println!("Finished {} score: {}", s, bound.0);
    self.t.put(curr_key, bound.0 as i8);
    Ok(bound.0)
  }

  pub fn reset(&mut self) {
    self.t.reset();
    self.count = 0;
  }

  pub fn solve(&mut self, moves: &Vec<u16>) -> Result<(State, i32), Box<dyn Error>> {
    let mut s = self.game.start();
    s.play_multiple(moves)?;
    let bound = s.bound();
    
    let score = self.negamax(&mut s, bound)?;
    
    Ok((s, score))
  }
}

#[cfg(test)]
mod test;