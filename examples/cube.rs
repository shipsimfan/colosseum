struct Scene;

fn main() {
    colosseum::run("Cube", Scene);
}

impl colosseum::Scene for Scene {
    fn update(&mut self) -> Option<Box<dyn colosseum::Scene>> {
        None
    }

    fn render(&mut self) {}
}
