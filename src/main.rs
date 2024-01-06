use std::{env, error::Error, path::{Path, PathBuf}, process};

use learn_connect4::{Solver, Connect4};

fn solve<P>(action_str: &str, to: Option<&P>, from: Option<&P>) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    
    let mut solver = match from {
        Some(from) => {
            let f = File::open(from)?;
            Solver::new_with_book(f)
        },
        None => {
            let game = Connect4::new(7, 6);
            Solver::new(game)
        }
    }?;
    let mut actions: Vec<u16> = Vec::with_capacity(solver.game().total_stones() as usize);
    
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
    if let Some(t) = to {
        let to_file = File::create(t)?;
        solver.write_to_book(to_file)?;
    }
    
    Ok(())
}

use std::fs::File;


fn cli() -> clap::Command {
    clap::Command::new("learn_connect4")
    .author("Zhifeng Wang, zhifeng5@illinois.edu")
    .about("A Connect4 Solver")
    .arg(clap::Arg::new("actions")
        .help("The actions string leading to the state you want to solve.")
    )
    .arg(clap::Arg::new("from")
      .long("from")
      .help("Loading an opening book")
      .value_parser(clap::value_parser!(PathBuf))
    )
    .arg(clap::Arg::new("to")
      .long("to")
      .help("Saving an opening book")
      .value_parser(clap::value_parser!(PathBuf))
    )
}

struct Config<'a> {
  actions: String,
  from: Option<&'a PathBuf>,
  to: Option<&'a PathBuf>
}

impl<'a> Config<'a> {
  pub fn new(matches: &'a clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
    let actions =  match matches.get_one::<String>("actions") {
        Some(s) => {
            s.clone()
        },
        None => {
            "".to_string()
        }
    };
    let from = matches.get_one::<PathBuf>("from");
    let to = matches.get_one::<PathBuf>("to");

    Ok(Config { actions, from, to })
  }
}

fn run(args_vec: Vec<String>) -> Result<(), Box<dyn Error>> {
    let matches = cli().get_matches_from(args_vec);
    let config = Config::new(&matches)?;
    
    solve(&config.actions, config.to, config.from)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(e) = run(args) {
        eprintln!("Sorry, there is an error: {e}");
        process::exit(1);
    }
//     let usage = r#"
// <actions to state> <Path: to book> <Path: from book>
// "#;
    
//     match match (args.get(1), args.get(2), args.get(3)) {
//         (Some(a), Some(to), from) => run(a, to, from),
//         _ => Err("Invalid arguments".into())
//     } {
//         Err(e) => println!("Error: {}\nUsage: {}", e, usage),
//         Ok(()) => ()
//     }
}
