pub fn board_bottom_mask(width: u8, height: u8) -> u64 {
  let mut res: u64 = 0;
  let col_shift = height + 1;
  for i in 0..width {
    res |= 1 << col_shift * i;
  }
  res
}

pub fn board_mask(width: u8, height: u8) -> u64 {
  board_bottom_mask(width, height) * ((1 << height)-1)
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_board_bottom_mask_w7h6() {
    let got = board_bottom_mask(7, 6);
    let expected = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;
    assert_eq!(got, expected);
  }

  #[test]
  fn check_board_mask_w7h6() {
    let got = board_mask(7, 6);
    let expected = 0b0111111_0111111_0111111_0111111_0111111_0111111_0111111;
    assert_eq!(got, expected);
  }
}