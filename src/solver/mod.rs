use std::{rc::Rc, error::Error, fs::File };

use crate::{Connect4, State, GameState};

use self::game_table::C4GameTable;

mod game_table;

pub struct Solver {
  t: C4GameTable,
  count: usize,
  game: Rc<Connect4>
}

use game_table::GameTableUsize::*;
use super::game::StateResult::{Immediate, Bounds};

impl Solver {
  pub fn new(game: Rc<Connect4>) -> Result<Self, Box<dyn Error>> {
    let sizes = vec![(14, Book(1000)), (14, Table(4000037)), (14, Table(4000037))];
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

  pub fn write_to_book(&self, f: File) -> Result<(), Box<dyn Error>> {
    
    self.t.write_to_book(f)?;
    Ok(())
  }

  pub fn new_with_book(f: File) -> Result<Self, Box<dyn Error>> {

    let sizes = vec![(14, Book(1000)), (14, Table(4000037)), (14, Table(4000037))];
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
    if let Some(b) = bound_opt {
      bound = b;
      match res {
        Immediate(v) => {
          // println!("Finished {} score: {}", s, new_bound.0);
          return Ok(v);
        },
        Bounds(new_bound) => {
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
          
          if let Some(v) = self.t.get(&s) {
            bound.1 = v as i32;
            if bound.0 >= bound.1 {
              return Ok(bound.1);
            }
          }
        }
      };
    } else {
      match res {
        Immediate(v) => {
          return Ok(v);
        },
        Bounds(new_bound) => {
          bound = new_bound;
        }
      }
    };
    
    
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
    self.t.put(&s, bound.0 as i8);
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