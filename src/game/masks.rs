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

fn get_vertical_winning_mask(my: &u64) -> u64 {
  (my << 1) & (my << 2) & (my << 3)
}

fn get_winning_mask_with_direction(shf: u8, my: &u64) -> u64 {

  let shf2 = shf * 2;
  let shf3 = shf * 3;

  let mut two = (my << shf) & (my << shf2);
  // ooo*
  let mut res = two & (my << shf3);
  // oo*o
  res |= two & (my >> shf);
  // *oo
  two = (my >> shf) & (my >> shf2);
  // *ooo
  res |= two & (my >> shf3);
  // o*oo
  res |= two & (my << shf);

  res
}

pub fn get_winning_mask(my: &u64, height: &u8) -> u64 {
  // Vertical winning positions: shifting up
  let mut res = get_vertical_winning_mask(&my);

  res |= get_winning_mask_with_direction(height + 1, my);

 res |= get_winning_mask_with_direction(*height, my);

 res |= get_winning_mask_with_direction(height + 2, my);

  res
}


#[cfg(test)]
mod tests {
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

  #[test]
  fn check_vertical_winning_w7h6() {
    let my: u64 = 0b0000111_0000111_0000111_0000111_0000111_0000111_0000111;
    let got = get_vertical_winning_mask(&my);
    let expected: u64 = 0b0001000_0001000_0001000_0001000_0001000_0001000_0001000;
    assert_eq!(got, expected);
  }

  #[test]
  fn check_horizontal_winning_w7h6() {
    let my: u64 = 0b0000000_0000111_0000111_0000111_0000000_0000000_0000111;
    let got = get_winning_mask_with_direction(7, &my);
    let expected: u64 = 0b0000111_0000000_0000000_0000000_0000111_0000000_0000000;

    assert_eq!(got, expected);
  }

  #[test]
  fn check_down_diag_winning_w7h6() {
    let my: u64 = 0b0000000_0000111_0000111_0000111_0000000_0000000_0000111;
    let got = get_winning_mask_with_direction(8, &my);
    let expected: u64 = 0b0001000_0000000_0000000_0000000_0000000_1000000_0000000;

    assert_eq!(got, expected);
  }

   #[test]
  fn check_up_diag_winning_w7h6() {
    let my: u64 = 0b0000000_0000111_0000111_0000111_0000000_0000000_0000111;
    let got = get_winning_mask_with_direction(6, &my);
    let expected: u64 = 0b0000000_1000000_0000000_0000000_0001000_0000000_0000000;

    assert_eq!(got, expected);
  }
}