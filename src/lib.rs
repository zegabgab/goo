pub mod script;

pub struct Game {
    name: String,
}

impl Game {
    pub fn new(name: String) -> Game {
        Game { name }
    }

    pub fn play(&mut self) {
        println!("Playing {}", self.name);
    }
}
