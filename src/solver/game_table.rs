use std::{collections::HashMap, rc::Rc, error::Error};

mod table;
use table::Table;

use super::NegamaxResult::{self, *};

use crate::{Connect4, State, GameState};

pub enum GameTableUsize {
  Book(usize),
  Table(usize)
}

type Book = HashMap<u64, NegamaxResult>;
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

  fn insert_key(&mut self, moves_num: &usize, key: u64, score: NegamaxResult) {
    match self.indices[*moves_num] {
      Book(i) => {
        let book = &mut self.books[i];
        if let Some(old_score) = book.get_mut(&key) {
          if let Pruned(_) = old_score {
            *old_score = score;
          }
        } else {
          book.insert(key, score);
        }
      },
      Table(i) => {
        let table = &mut self.tables[i];
        table.put(key, score);
      }
    }
  }

  pub fn put(&mut self, s: &State, score: NegamaxResult) {
    let moves_num = s.len();
    self.insert_key(&moves_num, s.key(), score);
  }

  pub fn get(&self, s: &State) -> Option<&NegamaxResult> {
    let moves_num = s.len();
    match self.indices[moves_num] {
      Book(i) => {
        let book = &self.books[i];
        book.get(&s.key())
      },
      Table(i) => {
        let table = &self.tables[i];
        table.get(s.key())
      }
    }
  }

  pub fn get_pruned_keys_in_books(&self) -> Vec<u64> {
    let mut ans: Vec<u64> = Vec::with_capacity(100);
    for book in self.books.iter() {
      for (key, res) in book.iter() {
        if let Pruned(_) = res {
          ans.push(*key);
        }
      }
    }
    ans
  }

  pub fn game(&self) -> Rc<Connect4> {
    Rc::clone(&self.game)
  }
}

mod build;

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn check_put_get() -> Result<(), Box<dyn Error>>{
    let game = Connect4::new(7, 6);
    let sizes: Vec<(usize, GameTableUsize)> = vec![(5, Book(100)), (37, Table(8388593))];
    let mut t = C4GameTable::new(Rc::clone(&game), sizes)?;
    let s = game.start();

    t.put(&s, NegamaxResult::Actual(-1));
    assert_eq!(t.get(&s), Some(NegamaxResult::Actual(-1)).as_ref());
    Ok(())
  }
  #[test]
  fn debug() {
    let a: i8 = -1;
    println!("{:b} -> {:b}", a, a.to_le_bytes()[0]);
  }
}