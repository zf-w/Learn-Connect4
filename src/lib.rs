mod game;

pub trait GameState {
  fn can_play(&self, a: u16) -> bool;
  fn possible_moves(&self) -> Vec<u16>;
  fn play(&mut self, a: u16) -> Result<(),Box<dyn Error>>;
  fn unplay(&mut self) -> Result<(), Box<dyn Error>>;
  fn key(&self) -> u64;
}

mod solver;

#[allow(dead_code)]
mod explorer;

use std::error::Error;

pub use game::Connect4;
pub use game::State;
pub use solver::Solver;
pub use explorer::Explorer;