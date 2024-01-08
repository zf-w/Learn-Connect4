#[derive(PartialEq, Debug)]
pub enum StateResult {
  Immediate(i32),
  Bounds((i32, i32))
}

use StateResult::{Immediate, Bounds};

impl super::State {
  pub fn my_left_stones(&self) -> u16 {
    let total_used = self.moves_num as u16;
    let total = self.game.total_stones();
    
    let first_hand = total_used % 2;
    if total % 2 == 1 {
      (total / 2 + first_hand) - (total_used / 2)
    } else {
      (total / 2) - (total_used / 2)
    }
  }

  pub fn opponent_left_stones(&self) -> u16 {
    let total_used = self.moves_num as u16;
    self.game.total_stones() - total_used - self.my_left_stones()
  }

  pub fn bound(&self) -> StateResult {
    let moves_mask = self.possible_moves_mask();
    let me_winning_mask = self.me_winning_mask();
    let op_next_winning_mask = self.opponent_winning_mask() & moves_mask;
    let my_left_stones = self.my_left_stones();
    let op_left_stones = self.opponent_left_stones();
    if me_winning_mask & moves_mask > 0 {
      return Immediate(my_left_stones as i32);
    }
    if (op_next_winning_mask > 0) && (op_next_winning_mask & (op_next_winning_mask - 1) > 0) {
      return Immediate(-(op_left_stones as i32));
    }
    if self.moves_num as u16 >= self.game.total_stones() {
      return Immediate(0);
    }

    Bounds((-((op_left_stones) as i32), (my_left_stones) as i32))
  }
}

#[cfg(test)]
mod test {
  use crate::{Connect4, game::state::{_format_board, bound::StateResult}};

  #[test]
  fn check_bound_case1() {
    let game = Connect4::new(7, 6);
    let mut s = game.start();
    let action_str = "2252576253462244111563365343671351441";
    let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
      c as u16 - '1' as u16
    }).collect();
    assert!(s.play_multiple(&actions).is_ok());

      assert_eq!(format!("{}", s), r#"
Log: 2252576253462244111563365343671351441
Num: 37
oxxxo..
xoxooo.
oxxoxx.
oxoxoo.
xxxooxx
ooxooox
"#);
    println!("{}", _format_board(s.me_winning_mask(), 7, 6));
    let expected = StateResult::Bounds((-2, 3));
    assert_eq!(s.bound(), expected);
  }

  #[test]
  fn check_bound_case2() {
    let game = Connect4::new(7, 6);
    let mut s = game.start();
    let action_str = "6763525635134453444361412671365712277";
    let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
      c as u16 - '1' as u16
    }).collect();
    assert!(s.play_multiple(&actions).is_ok());

      assert_eq!(format!("{}", s), r#"
Log: 6763525635134453444361412671365712277
Num: 37
..oo.x.
o.xooxo
xoxxoox
xxxoxxx
xooxooo
oxxooox
"#);
    let expected = StateResult::Bounds((-2, 3));
    assert_eq!(s.bound(), expected);
  }
}