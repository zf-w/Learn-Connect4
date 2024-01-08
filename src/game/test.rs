use crate::GameState;

use super::*;
#[test]
fn col_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b111111, game.mask_col(0));
  assert_eq!(0b111111_0000000, game.mask_col(1));
  assert_eq!(0b111111_0000000_0000000, game.mask_col(2));
}

#[test]
fn col_top_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b100000, game.mask_col_top(0));
  assert_eq!(0b100000_0000000, game.mask_col_top(1));
}

#[test]
fn col_bottom_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b1, game.mask_col_bottom(0));
  assert_eq!(0b1_0000000, game.mask_col_bottom(1));
}

#[test]
fn check_flip_board_for_w7h6() {
  let game = Connect4::new(7, 6);
  let ipt: u64 = 0b0111111_0011111_0001111_0000111_0000011_0000001_0000000;
  let expected: u64 = 0b0000000_0000001_0000011_0000111_0001111_0011111_0111111;
  assert_eq!(game.flip_c4_board(ipt), expected);
}

#[test]
fn check_display() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  assert_eq!(format!("{}", s), r#"
Log: 
Num: 0
.......
.......
.......
.......
.......
.......
"#);
 assert!(s.play(1).is_ok());
 assert_eq!(format!("{}", s), r#"
Log: 2
Num: 1
.......
.......
.......
.......
.......
.o.....
"#);
  assert!(s.play(1).is_ok());
  assert!(s.play(1).is_ok());
  assert!(s.play(1).is_ok());
  assert!(s.play(1).is_ok());
  assert!(s.play(1).is_ok());
  assert_eq!(format!("{}", s), r#"
Log: 222222
Num: 6
.x.....
.o.....
.x.....
.o.....
.x.....
.o.....
"#);
  assert!(s.play(1).is_err());
}

#[test]
fn check_retrieve_player_and_mask() {
  let game = Connect4::new(7, 6);
  let ipt: u64 = 0b0000001_0000001_0000010_0000001_0000001_0000001_0000001;
  let expected_mask: u64 = 0b0000000_0000000_0000001_0000000_0000000_0000000_0000000;
  let expected_player: u64 = 0b0000000_0000000_0000000_0000000_0000000_0000000_0000000;
  assert_eq!(game.build_mask_and_player_from_combined(ipt), (expected_mask, expected_player, 1));
}