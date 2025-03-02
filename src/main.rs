use goo::Game;
use goo::script::Interpreter;

fn main() {
    let mut game = Game::new(String::from("exmpl"));
    game.play();

    Interpreter::new().execute(String::from("[] {\n\tglob helloo = 69\n\tout helloo\n}"));
}
