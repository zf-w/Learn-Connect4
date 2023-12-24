use learn_connect4::GameState;

fn main() {
    let game = learn_connect4::Connect4::new(7, 6);
    let mut s = game.start();
    println!("{}", s);
    s.play(0).expect("Should work");
    println!("{}", s);
    s.play(0).expect("Should work");
    println!("{}", s);
}
