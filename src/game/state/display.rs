use std::fmt::Display;

impl Display for super::State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.game.width;
        let h = self.game.height;
        let mut stones_str: String = String::with_capacity((w * h + h) as usize + 10);
        let mut moves_str: String = String::with_capacity(self.moves.len());
        for a in self.moves.iter() {
          moves_str.push((a + '1' as u8) as char);
        }
        stones_str.push_str(&format!("\nLog: {}\nNum: {}\n", moves_str, self.moves.len()));
        
        let first_to_play: bool = self.moves.len() % 2 == 0;
        for row_i in 0..h {
          let h_offset = h - 1 - row_i;
          
          let mask = self.mask;
          for col_i in 0..w {
            let offset = h_offset + (h + 1) * col_i;
            // let player = ((self.player >> offset) & 1 == 0) ^ first_to_play;
            let to_push = if mask >> offset & 1 == 1 {
              match ((self.player >> offset) & 1 == 1, first_to_play) {
                (true, true) => 'o',
                (false, true) => 'x',
                (true, false) => 'x',
                (false, false) => 'o'
              }
            } else {
              '.'
            };
            stones_str.push(to_push);
          }
          stones_str.push('\n');
        }
        write!(f, "{}", stones_str)
    }
}