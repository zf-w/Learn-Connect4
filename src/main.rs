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
        Ok(v) => {
            println!("Final score: {}", v);
        },
        Err(s) => {
            println!("Error: {}", s);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    if args.len() < 2 {
        eprintln!("Application error: not enough arguments");
        process::exit(1);
    }
    run(&args[1]);
}
