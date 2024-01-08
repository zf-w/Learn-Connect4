use std::{rc::Rc, error::Error, fs::File };

use crate::{Connect4, State, GameState};

use self::game_table::C4GameTable;

mod game_table;

pub struct Solver {
  t: C4GameTable,
  count: usize,
  game: Rc<Connect4>
}

type Score = i8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NegamaxResult {
  Actual(Score),
  Pruned(Score)
}

use NegamaxResult::*;

use game_table::GameTableUsize::*;
use super::game::StateResult::{Immediate, Bounds};

impl Solver {
  pub fn new(game: Rc<Connect4>) -> Result<Self, Box<dyn Error>> {
    let sizes = vec![(12, Book(1000)), (15, Table(4000037)), (15, Table(4000037))];
    let t: C4GameTable = C4GameTable::new(Rc::clone(&game), sizes)?;
    Ok(Self {
      t,
      count: 0,
      game
    })
  }

  pub fn count(&self) -> usize {
    self.count
  }

  pub fn finalize_pruned_in_books(&mut self) -> Result<bool, Box<dyn Error>> {
    let wait = self.t.get_pruned_keys_in_books();
    let len = wait.len();
    println!("Finalizing {} states...", wait.len());
    for (i, key) in wait.iter().enumerate() {
      let mut s = self.game.start_from_mask(*key);
      let score = self.negamax(&mut s, None)?;
      print!("\r{} / {}", i, len);
      self.t.put(&s, Actual(score as i8));
    }
    println!("");

    Ok(len > 0)
  }

  pub fn write_to_book(&self, f: File) -> Result<(), Box<dyn Error>> {
    
    self.t.write_to_book(f)?;
    Ok(())
  }

  pub fn new_with_book(f: File) -> Result<Self, Box<dyn Error>> {

    let sizes = vec![(12, Book(1000)), (15, Table(4000037)), (15, Table(4000037))];
    let t: C4GameTable = C4GameTable::new_with_book(f, sizes)?;
    let game = t.game();
    Ok(Self {
      t, game, count: 0
    })
  }

  fn negamax(&mut self, s: &mut State, bound_opt: Option<(i32, i32)>) -> Result<i32, Box<dyn Error>> {
    // println!("Starting {}", s);
    self.count += 1;
    let res = s.bound();
    let mut bound: (i32, i32);
    match res {
      Immediate(v) => {
        // println!("Finished {} score: {}", s, new_bound.0);
        return Ok(v);
      },
      Bounds(new_bound) => {
        if let Some(b) = bound_opt {
          bound = b;
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
        } else {
          bound = new_bound;
        }
      }
    };

    if let Some(res) = self.t.get(&s) {
      match res {
        Actual(s) => {
          return Ok(*s as i32);
        },
        Pruned(s) => {
          bound.1 = *s as i32;
          if bound.0 >= bound.1 {
            return Ok(bound.1);
          }
        },
      }
    }
    
    // println!("Looping with [{}, {}]", bound.0, bound.1);
    let actions = s.nonlosing_moves_sorted();
    for a in actions.iter() {
      s.play(*a)?;
      let score = -self.negamax(s, Some((-bound.1, -bound.0)))?;
      s.unplay()?;
      if score > bound.0 {
        bound.0 = score;
      }
      if score >= bound.1 {
        // self.t.put(curr_key, score as i8);
        
        return Ok(score);
      }
    }
    // println!("Finished {} score: {}", s, bound.0);
    match bound_opt {
      None => {
        self.t.put(&s, Actual(bound.0 as i8));
      },
      Some(_) => {
        self.t.put(&s, Pruned(bound.0 as i8));
      }
    }
    
    Ok(bound.0)
  }

  pub fn reset(&mut self) {
    self.t.reset();
    self.count = 0;
  }

  pub fn solve(&mut self, moves: &Vec<u16>) -> Result<(State, i32), Box<dyn Error>> {
    let mut s = self.game.start();
    s.play_multiple(moves)?;
    let res = s.bound();

    match res {
      Immediate(score) => {
        return Ok((s, score));
      },
      Bounds(bound) => {
        let score = self.negamax(&mut s, Some(bound))?;
        return Ok((s, score));
      }
    };
  }

  pub fn log_from_start(&mut self) -> Result<(), Box<dyn Error>> {
    let mut s = self.game.start();

    // let bound = s.bound();
   self.negamax(&mut s, None)?;
    Ok(())
  }

  pub fn game(&self) -> Rc<Connect4> {
    Rc::clone(&self.game)
  }
}

#[cfg(test)]
mod test;