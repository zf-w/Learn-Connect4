use std::{fs, path::Path};

use crate::{Solver, Connect4};

#[test]
fn bebug_place() {
  let game = Connect4::new(7, 6);
  let mut solver = Solver::new(game.clone());

  let mut s = game.start();
  let action_str = "6763525635134453444361412671365712277";
  let actions: Vec<u16> = action_str.chars().map(|c| -> u16 {
    c as u16 - '1' as u16
  }).collect();
  assert!(s.play_multiple(&actions).is_ok());
  println!("{}", s);
  println!("{:?}", s.nonlosing_moves_sorted());
  println!("[{} {}]", s.bound().0, s.bound().1);
  println!("{}", solver.negamax(&mut s, (-2, 3)).unwrap());
}

fn read_to_string<P>(p: P, f: &mut String) -> Result<(), Box<dyn std::error::Error>>
  where P: AsRef<Path> {
  *f = fs::read_to_string(p)?;
  Ok(())
}

fn read_moves_to_vec(action_str: &str, actions: &mut Vec<u16>) {
  for c in action_str.chars() {
      actions.push(c as u16 - '1' as u16);
  }
}

fn check_state(actions_str: &str, expected_str: &str, actions: &mut Vec<u16>, solver: &mut Solver) {
  solver.reset();
  actions.clear();
  read_moves_to_vec(actions_str, actions);
  let res = solver.solve(&actions);
  assert!(res.is_ok());
  let expected_res = expected_str.parse::<i32>();
  assert!(expected_res.is_ok());
  assert_eq!(res.unwrap().1, expected_res.unwrap(), "Testing {}", actions_str);
}

fn parse_testcase_string(f: &str) -> Vec<(&str, &str)> {
  let count = f.chars().filter(|c| *c == '\n').count();
  let mut res: Vec<(&str, &str)> = Vec::with_capacity(count + 2);
  let mut prev_start: usize = 0;
  let mut prev_space: usize = 0;
  for (i, c) in f.char_indices() {
    match c {
      ' ' => {
        prev_space = i;
      },
      '\n' => {
        // count += 1;
        let actions_str = &f[prev_start..prev_space];
        let expected_str = &f[(prev_space + 1)..i];
        res.push((actions_str, expected_str));
        prev_start = i + 1;
      },
      _ => ()
    }
  }
  res
}

/// 6763525635134453444361412671365712277
#[test]
fn check_l3_r1() {
  let mut f: String = String::new();
  assert!(read_to_string("./src/testcases/Test_L3_R1", &mut f).is_ok());
  
  let game = Connect4::new(7, 6);
  let mut solver = Solver::new(game);
  let mut actions: Vec<u16> = Vec::with_capacity(7 * 6);
  // let mut count = 0;
  let cases = parse_testcase_string(&f);
  println!("Total number of cases: {}...", cases.len());
  for (actions_str, expected_str) in cases {
    check_state(actions_str, expected_str, &mut actions, &mut solver);
  }
}

#[test]
fn check_l2_r1() {
  let mut f: String = String::new();
  assert!(read_to_string("./src/testcases/Test_L2_R1", &mut f).is_ok());
  
  let game = Connect4::new(7, 6);
  let mut solver = Solver::new(game);
  let mut actions: Vec<u16> = Vec::with_capacity(7 * 6);
  // let mut count = 0;
  let cases = parse_testcase_string(&f);
  println!("Total number of cases: {}...", cases.len());
  for (actions_str, expected_str) in cases {
    check_state(actions_str, expected_str, &mut actions, &mut solver);
  }
}