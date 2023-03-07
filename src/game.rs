use crate::{InitialScene, Scene};

pub struct Game {
    scene: Box<dyn Scene>,
}

impl Game {
    pub fn new<S: InitialScene>(context: S::Context) -> Self {
        Game {
            scene: S::new(context),
        }
    }

    pub fn run() {}
}
