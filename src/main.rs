use std::{rc, env, process};

use learn_connect4::Solver;
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Application error: not enough arguments");
        process::exit(1);
    }
    println!("{}", &args[1]);
    run(&args[1]);
}
