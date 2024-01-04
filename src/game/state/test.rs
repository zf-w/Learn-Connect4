use std::error::Error;

use crate::{Connect4, GameState, game::state::_format_board};

#[test]
fn check_possible_moves() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let got0 = s.possible_moves_mask();
  let expected0: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;
  assert_eq!(got0, expected0);
  assert!(s.play(2).is_ok());
  let got1 = s.possible_moves_mask();
  let expected1: u64 = 0b0000001_0000001_0000001_0000001_0000010_0000001_0000001;
  assert_eq!(got1, expected1);
}

#[test]
fn check_play_multiple() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0];
  assert!(s.play_multiple(&actions).is_ok());
  assert_eq!(format!("{}", s), r#"
Log: 44455554221
Num: 11
.......
.......
...xo..
...ox..
.x.xo..
oo.ox..
"#);
}

#[test]
fn check_empty_space_mask() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0];
  assert!(s.play_multiple(&actions).is_ok());
  let expected: u64 = 
  0b0111111_0111111_0110000_0110000_0111111_0111100_0111110;
  
  let got = s.empty_space_mask();
  
  assert_eq!(got, expected);
}

#[test]
fn check_me_winning_mask() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0,2];
  assert!(s.play_multiple(&actions).is_ok());
  let expected: u64 = 
  0b10_0000000_0000000;
  
  let got = s.me_winning_mask();
  
  assert_eq!(got, expected);
}

#[test]
fn check_opponent_winning_mask() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0];
  assert!(s.play_multiple(&actions).is_ok());
  let expected: u64 = 
  0b11_0000000_0000000;
  
  let got = s.opponent_winning_mask();
  
  assert_eq!(got, expected);
}

#[test]
fn check_is_winning_action() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0,2];
  assert!(s.play_multiple(&actions).is_ok());
  
  assert_eq!(s.is_winning_action(2), true);
}

#[test]
fn check_heuristic_action_score() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0];
  let a: Vec<u16> = vec![0,1,2,3,4,5,6];
  assert!(s.play_multiple(&actions).is_ok());
  
  let got: Vec<u16> = a.iter().map(
    |a| -> u16 {
      s.heuristic_action_score(*a)
  }).collect();

// .......
// .......
// ...xo..
// ...ox..
// .x.xo..
// oo.ox..
  let expected = vec![4,1,6,3,6,1,4];
  assert_eq!(got, expected)
}

#[test]
fn check_nonlosing_moves_sorted_case_losing() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,3,4,4,4,4,3,1,1,0];
  assert!(s.play_multiple(&actions).is_ok());
 
  let expected_sorted_moves: Vec<u16> = vec![2];
  assert_eq!(s.nonlosing_moves_sorted(), expected_sorted_moves);
}

#[test]
fn check_nonlosing_moves_sorted_case_start() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let actions: Vec<u16> = vec![3,3,4,2,4];
  let a: Vec<u16> = vec![0,1,2,3,4,5,6];
  assert!(s.play_multiple(&actions).is_ok());
  
  let got_scores: Vec<u16> = a.iter().map(
    |a| -> u16 {
      s.heuristic_action_score(*a)
  }).collect();
  let expected_scores: Vec<u16> = vec![0,1,2,3,6,1,0];

  assert_eq!(format!("{}", s), r#"
Log: 44535
Num: 5
.......
.......
.......
.......
...xo..
..xoo..
"#);

  assert_eq!(got_scores, expected_scores);
  let expected_sorted_moves: Vec<u16> = vec![4,3,2,1,5,0,6];
  assert_eq!(s.nonlosing_moves_sorted(), expected_sorted_moves);
}

#[test]
fn check_bug1_wrong_filp_in_unplay() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  let action_str = "225257625346224411156336534367135144";
  let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
    c as u16 - '1' as u16
  }).collect();
  assert!(s.play_multiple(&actions).is_ok());

  assert_eq!(format!("{}", s), r#"
Log: 225257625346224411156336534367135144
Num: 36
.xxxo..
xoxooo.
oxxoxx.
oxoxoo.
xxxooxx
ooxooox
"#);
// let keep = stone_mask ^ 1; used 0 here, causing error
// self.mask &= keep;
// self.player &= keep;
// self.player ^= self.game.mask_full(); 
//
//
  // println!("{}", format_board(s.player, game.width, game.height));
  // println!("{}", format_board(s.mask, game.width, game.height));
  assert!(s.play(0).is_ok());
  // println!("{}", format_board(s.player, game.width, game.height));
  // println!("{}", format_board(s.mask, game.width, game.height));
  assert!(s.unplay().is_ok());
  // println!("{}", s);
  // println!("{}", format_board(s.player, game.width, game.height));
  // println!("{}", format_board(s.mask, game.width, game.height));
  assert_eq!(format!("{}", s), r#"
Log: 225257625346224411156336534367135144
Num: 36
.xxxo..
xoxooo.
oxxoxx.
oxoxoo.
xxxooxx
ooxooox
"#);
}

#[test]
fn check_bug2_key_symmetric() -> Result<(), Box<dyn Error>> {
  
// .......
// .......
// .......
// .......
// ...xo..
// ..xoo..
  let w: u8 = 7;
  let h: u8 = 6;
  let game = Connect4::new(7, 6);
  let mut s0 = game.start();
  let mut s1 = game.start();
  let actions0: Vec<u16> = vec![3,3,4,2,4,0,1];
  let actions1: Vec<u16> = vec![3,3,2,4,2,6,5];
  s0.play_multiple(&actions0)?;
  s1.play_multiple(&actions1)?;
  // println!("{}", s0);
  // println!("{}", s1);
  println!("{}", _format_board(s0.key(), w, h));
  println!("{}", _format_board(s1.key(), w, h));
  assert_eq!(s0.key(), s1.key());
  Ok(())
}

#[test]
fn check_count_stones_mask() {
  let ipt = 0b0111111_0111111_0110000_0110000_0111111_0111100_0111110u64;
  let expected = 15usize;
  let game = Connect4::new(7, 6);
  assert_eq!(game.count_mask_stones(ipt), expected);
}