use std::{rc::Rc, error::Error};

use crate::{Connect4, State, GameState};

use self::transposition::TranspositionTable;

mod transposition;

pub struct Solver {
  t: TranspositionTable,
  game: Rc<Connect4>
}

impl Solver {
  pub fn new(game: Rc<Connect4>) -> Self {
    Self {
      t: TranspositionTable::new(8388593),
      game
    }
  }

  fn negamax(&mut self, s: &mut State, mut bound: (i32, i32)) -> Result<i32, Box<dyn Error>> {
    let new_bound = s.bound();
    if new_bound.0 == new_bound.1 {
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

    let actions = s.nonlosing_moves_sorted();
    for a in actions.iter() {
      s.play(*a)?;
      let score = -self.negamax(s, bound)?;
      s.unplay()?;
      if score >= bound.1 {
        return Ok(score);
      }
      if score > bound.0 {
        bound.0 = score;
      }
      
    }
    self.t.put(curr_key, bound.0 as i8);
    Ok(bound.0)
  }

  pub fn solve(&mut self, moves: &Vec<u16>) -> Result<i32, Box<dyn Error>> {
    let mut s = self.game.start();
    s.play_multiple(moves)?;
    let bound = s.bound();
    println!("{}", s);
    let score = self.negamax(&mut s, bound)?;
    
    Ok(score)
  }
}

#[cfg(test)]
mod test {
    use crate::{Solver, Connect4, GameState};

  #[test]
  fn check_solve() {
    let game = Connect4::new(7, 6);
    let mut solver = Solver::new(game.clone());
    
    let mut s = game.start();
    let action_str = "2252576253462244111563365343671351441";
    let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
      c as u16 - '1' as u16
    }).collect();
    assert!(s.play_multiple(&actions).is_ok());
    assert!(s.play(6).is_ok());
    println!("{}", solver.negamax(&mut s, (-1, 2)).unwrap());
  }
}