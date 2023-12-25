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
fn check_display() {
  let game = Connect4::new(7, 6);
  let mut s = game.start();
  assert_eq!(format!("{}", s), r#"
.......
.......
.......
.......
.......
.......
"#);
 assert!(s.play(1).is_ok());
 assert_eq!(format!("{}", s), r#"
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
.x.....
.o.....
.x.....
.o.....
.x.....
.o.....
"#);
  assert!(s.play(1).is_err());
}