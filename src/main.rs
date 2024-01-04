use std::{rc, env, error::Error};

use learn_connect4::{Solver, Explorer};

fn run(action_str: &str) {
    let game = learn_connect4::Connect4::new(7, 6);
    let mut actions: Vec<u16> = Vec::with_capacity(game.total_stones() as usize);
    let mut solver = Solver::new(rc::Rc::clone(&game));
    for c in action_str.chars() {
        actions.push(c as u16 - '1' as u16);
    }
    
    match solver.solve(&actions) {
        Ok((s, v)) => {
            println!("{}", s);
            println!("\"{}\"'s turn", if actions.len() % 2 == 0 {
                'o'
            } else {
                'x'
            });
            println!("Score: {}", v);
            println!("Result: {}", 
            if v == 0 {
                String::from("draw")
            } else if v > 0 {
                format!("win in {} turns", s.my_left_stones() as i32 - v)
              
            } else {
                format!("lose in {} turns", s.opponent_left_stones() as i32 + v)
            });
            println!("Expored nodes: {}", solver.count());
        },
        Err(s) => {
            println!("Error: {}", s);
        }
    }
}

fn make_book() -> Result<(), Box<dyn Error>>{
    let game = learn_connect4::Connect4::new(7, 6);
    let mut explorer = Explorer::new(rc::Rc::clone(&game));
    explorer.log_from_start()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
        println!("{}", &args[1]);
    if let Some(v) = args.get(1) {
        if v == "make" {
            match make_book() {
                _ => ()
            }
        } else {
            run(v);
        }
    } else {
        println!("Empty (start state)");
        run(&"");
    }
}
