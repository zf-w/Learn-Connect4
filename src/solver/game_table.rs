use std::{collections::HashMap, rc::Rc, error::Error, fs::File, io::{Write, Read}};

mod table;
use table::Table;

use crate::{Connect4, State, GameState};

pub enum GameTableUsize {
  Book(usize),
  Table(usize)
}

type Book = HashMap<u64, i8>;
pub struct C4GameTable {
  game: Rc<Connect4>,
  indices: Vec<GameTableUsize>,
  books: Vec<Book>,
  tables: Vec<Table>,
  write_books: bool
}

use GameTableUsize::*;

impl C4GameTable {
  pub fn new(game: Rc<Connect4>, sizes: Vec<(usize, GameTableUsize)>) -> Result<Self, Box<dyn Error>> {
    let mut books_len: usize = 0;
    let mut tables_len: usize = 0;
    let total = game.total_stones() as usize;
    let mut indices: Vec<GameTableUsize> = Vec::with_capacity(total);
    for (size, v) in sizes.iter() {
      match v {
        Book(_) => {
          for _ in 0..*size {
            indices.push(Book(books_len));
          }
          books_len += 1;
        },
        Table(_) => {
          for _ in 0..*size {
            indices.push(Table(tables_len));
          }
          tables_len += 1;
        }
      }
    }
    if indices.len() != total {
      return Err("The tables definition given does not match the total turns' number".into());
    }
    
    let mut books: Vec<Book> = Vec::with_capacity(books_len);
    let mut tables: Vec<Table> = Vec::with_capacity(tables_len);
    for (_, v) in sizes.iter() {
      match v {
        Book(v) => {
          books.push(HashMap::with_capacity(*v));
        },
        Table(v) => {
          tables.push(Table::new(*v));
        }
      }
    }
    Ok(Self {
      game,
      indices,
      books,
      tables,
      write_books: true
    })
  }

  pub fn reset(&mut self) {
    for table in self.tables.iter_mut() {
      table.reset();
    }
  }

  fn insert_key(&mut self, moves_num: &usize, key: u64, score: i8) {
    match self.indices[*moves_num] {
      Book(i) => {
        let book = &mut self.books[i];
        book.insert(key, score);
      },
      Table(i) => {
        let table = &mut self.tables[i];
        table.put(key, score);
      }
    }
  }

  pub fn put(&mut self, s: &State, score: i8) {
    let moves_num = s.len();
    self.insert_key(&moves_num, s.key(), score);
  }

  pub fn get(&self, s: &State) -> Option<i8> {
    let moves_num = s.len();
    match self.indices[moves_num] {
      Book(i) => {
        let book = &self.books[i];
        book.get(&s.key()).copied()
      },
      Table(i) => {
        let table = &self.tables[i];
        table.get(s.key())
      }
    }
  }

  pub fn game(&self) -> Rc<Connect4> {
    Rc::clone(&self.game)
  }

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
      for (case, score) in book.iter() {
        push_u64_to_bytes(&mut to_write, case);
        push_i8_to_bytes(&mut to_write, score);
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
    println!("{}, {}", w, h);
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
      me.insert_key(&count, case, score);
    }
    me.write_books = false;
    Ok(me)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn check_put_get() -> Result<(), Box<dyn Error>>{
    let game = Connect4::new(7, 6);
    let sizes: Vec<(usize, GameTableUsize)> = vec![(5, Book(100)), (37, Table(8388593))];
    let mut t = C4GameTable::new(Rc::clone(&game), sizes)?;
    let s = game.start();

    t.put(&s, -1);
    assert_eq!(t.get(&s), Some(-1));
    Ok(())
  }
  #[test]
  fn debug() {
    let a: i8 = -1;
    println!("{:b} -> {:b}", a, a.to_le_bytes()[0]);
  }
}