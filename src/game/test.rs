use super::*;
#[test]
fn col_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b111111, game.col_mask(0));
  assert_eq!(0b111111_0000000, game.col_mask(1));
  assert_eq!(0b111111_0000000_0000000, game.col_mask(2));
}

#[test]
fn col_top_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b100000, game.col_top_mask(0));
  assert_eq!(0b100000_0000000, game.col_top_mask(1));
}

#[test]
fn col_bottom_mask_check_for_w7h6() {
  let game = Connect4::new(7, 6);
  assert_eq!(0b1, game.col_bottom_mask(0));
  assert_eq!(0b1_0000000, game.col_bottom_mask(1));
}