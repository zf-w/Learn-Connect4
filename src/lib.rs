mod game;

pub trait GameState {
  fn can_play(&self, a: u16) -> bool;
  fn possible_moves(&self) -> Vec<u16>;
  fn bound(&self) -> (i32, i32);
  fn play(&mut self, a: u16) -> Result<(),&'static str>;
  fn unplay(&mut self) -> Result<(), &'static str>;
  fn key(&self) -> u64;
}

pub use game::Connect4;
pub use game::State;