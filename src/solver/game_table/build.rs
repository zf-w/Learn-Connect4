use std::{rc::Rc, error::Error, fs::File, io::{Write, Read}};
use crate::Connect4;

use super::{C4GameTable, GameTableUsize};
use super::NegamaxResult::*;

impl C4GameTable {
  pub fn write_to_book(&self, mut f: File) -> Result<(), Box<dyn Error>> {
    fn push_u64_to_bytes(v: &mut Vec<u8>, it: &u64) {
      for byte in it.to_be_bytes() {
        v.push(byte);
      }
    }
    fn push_i8_to_bytes(v: &mut Vec<u8>, it: &i8) {
      for byte in it.to_be_bytes() {
        v.push(byte);
      }
    }
    fn push_u8_to_bytes(v: &mut Vec<u8>, it: &u8) {
      for byte in it.to_be_bytes() {
        v.push(byte);
      }
    }
    let w = self.game.width();
    let h = self.game.height();
    let total_cases: u64 = self.books.iter()
      .map(|book| -> usize {book.len()})
      .sum::<usize>() as u64;
    println!("Writing {} openings", {total_cases});
    let mut to_write: Vec<u8> = Vec::with_capacity(1 + 1 + 8 + 9 * total_cases as usize);
    push_u8_to_bytes(&mut to_write, &w);
    push_u8_to_bytes(&mut to_write, &h);
    push_u64_to_bytes(&mut to_write, &total_cases);
    for book in self.books.iter() {
      for (case, res) in book.iter() {
        if let Actual(score) = res {
          push_u64_to_bytes(&mut to_write, case);
          push_i8_to_bytes(&mut to_write, score);
        }
      }
    }
    
    f.write_all(&to_write)?;
    Ok(())
  }

  pub fn new_with_book(mut reader: File, sizes: Vec<(usize, GameTableUsize)>) -> Result<Self, Box<dyn Error>> {
    let mut buf8: [u8; 1] = [0; 1];
    let mut buf64: [u8; 8] = [0; 8];

    reader.read_exact(&mut buf8)?;
    let w = u8::from_be_bytes(buf8);
    reader.read_exact(&mut buf8)?;
    let h = u8::from_be_bytes(buf8);
  
    let game = Connect4::new(w, h);
    let mut me = Self::new(Rc::clone(&game), sizes)?;
    reader.read_exact(&mut buf64)?;
    let total = u64::from_be_bytes(buf64);
    println!("Number of stored: {}", total);
    for _ in 0..total {
      reader.read_exact(&mut buf64)?;
      let case = u64::from_be_bytes(buf64);
      reader.read_exact(&mut buf8)?;
      let score = i8::from_be_bytes(buf8);
      let count = game.count_mask_stones(case);
      me.insert_key(&count, case, Actual(score));
    }
    me.write_books = false;
    Ok(me)
  }
}