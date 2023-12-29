use crate::{Connect4, GameState};

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
 
  assert!(s.nonlosing_moves_sorted().is_empty());
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