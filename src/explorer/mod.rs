use std::{rc::Rc, error::Error, collections::HashMap};

use crate::{Connect4, State, GameState, game::_format_board};

use self::transposition::TranspositionTable;

mod transposition;

pub struct Explorer {
  t: TranspositionTable,
  count: usize,
  game: Rc<Connect4>,
  book: HashMap<u64, i8>
}

impl Explorer {
  pub fn new(game: Rc<Connect4>) -> Self {
    Self {
      t: TranspositionTable::new(8388593),
      count: 0,
      game,
      book: HashMap::with_capacity(1 << 23)
    }
  }

  pub fn count(&self) -> usize {
    self.count
  }

  fn negamax(&mut self, s: &mut State, mut bound: (i32, i32), log: bool) -> Result<i32, Box<dyn Error>> {
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
    if bound.1 > new_bound.1 {
      bound.1 = new_bound.1;
      if bound.0 >= bound.1 {
        return Ok(bound.1);
      }
    }
    let curr_key = s.key();

    let t = &self.t;

    if let Some(v) = t.get(curr_key) {
      bound.1 = v as i32;
      if bound.0 >= bound.1 {
        return Ok(bound.1);
      }
    }
    // println!("Looping with [{}, {}]", bound.0, bound.1);
    let actions = s.nonlosing_moves_sorted();
    for a in actions.iter() {
      s.play(*a)?;
      let score = -self.negamax(s, (-bound.1, -bound.0), log)?;
      s.unplay()?;
      if score > bound.0 {
        bound.0 = score;
      }
      if score >= bound.1 {
        // self.t.put(curr_key, bound.0 as i8);
        return Ok(score);
      }
    }
    // println!("Finished {} score: {}", s, bound.0);
    if log && s.len() < 14 {
      self.book.insert(curr_key, bound.0 as i8);
    }
    
    self.t.put(curr_key, bound.0 as i8, s.len() < 10);
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
    
    let score = self.negamax(&mut s, bound, false)?;
    
    Ok((s, score))
  }

  pub fn log_from_start(&mut self) -> Result<(), Box<dyn Error>> {
    let mut s = self.game.start();

    let bound = s.bound();
   self.negamax(&mut s, bound, true)?;
    Ok(())
  }

  pub fn compile_book(&self) {
    let mut count = 0;
    let w = self.game.width();
    let h = self.game.height();
    for (k, _) in self.book.iter() {
      println!("{}", _format_board(*k, w, h));
      if count > 10 {
        break;
      } else {
        count += 1;
      }
    }
  }
}

#[cfg(test)]
mod test;