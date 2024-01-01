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
    // println!("Starting {}", s);
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
      if score >= bound.1 {
        return Ok(score);
      }
      if score > bound.0 {
        bound.0 = score;
      }
      
    }
    self.t.put(curr_key, bound.0 as i8);
    // println!("Finished {} score: {}", s, bound.0);
    Ok(bound.0)
  }

  pub fn reset(&mut self) {
    self.t.reset();
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
mod test {
    use std::{fs, path::Path};

    use crate::{Solver, Connect4};

  #[test]
  fn bebug_place() {
    let game = Connect4::new(7, 6);
    let mut solver = Solver::new(game.clone());
    
    let mut s = game.start();
    let action_str = "6763525635134453444361412671365712277";
    let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
      c as u16 - '1' as u16
    }).collect();
    assert!(s.play_multiple(&actions).is_ok());
    println!("{}", s);
    println!("{:?}", s.nonlosing_moves_sorted());
    println!("[{} {}]", s.bound().0, s.bound().1);
    println!("{}", solver.negamax(&mut s, (-2, 3)).unwrap());
  }

  fn read_to_string<P>(p: P, f: &mut String) -> Result<(), Box<dyn std::error::Error>>
    where P: AsRef<Path> {
    *f = fs::read_to_string(p)?;
    Ok(())
  }

  fn read_moves_to_vec(action_str: &str, actions: &mut Vec<u16>) {
    for c in action_str.chars() {
        actions.push(c as u16 - '1' as u16);
    }
  }

  fn check_state(actions_str: &str, expected_str: &str, actions: &mut Vec<u16>, solver: &mut Solver) {
    solver.reset();
    actions.clear();
    read_moves_to_vec(actions_str, actions);
    let res = solver.solve(&actions);
    assert!(res.is_ok());
    let expected_res = expected_str.parse::<i32>();
    assert!(expected_res.is_ok());
    assert_eq!(res.unwrap().1, expected_res.unwrap(), "Testing {}", actions_str);
  }

  /// 6763525635134453444361412671365712277
  #[test]
  fn check_l3_r1() {
    let mut f: String = String::new();
    assert!(read_to_string("./src/testcases/Test_L3_R1", &mut f).is_ok());
    let mut prev_start: usize = 0;
    let mut prev_space: usize = 0;
    let game = Connect4::new(7, 6);
    let mut solver = Solver::new(game);
    let mut actions: Vec<u16> = Vec::with_capacity(7 * 6);
    let mut count = 0;
    for (i, c) in f.char_indices() {
      match c {
        ' ' => {
          prev_space = i;
        },
        '\n' => {
          count += 1;
          let actions_str: &str = &f[prev_start..prev_space];
          let expected_str = &f[(prev_space + 1)..i];
          check_state(actions_str, expected_str, &mut actions, &mut solver);
          prev_start = i + 1;
        },
        _ => ()
      }
      if count > 10 {
        break;
      }
    }
  }
}