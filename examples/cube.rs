#[allow(unused)]
struct Scene(colosseum::logging::Logger);

fn main() {
    colosseum::run("Cube", None, Scene::new);
}

impl Scene {
    pub fn new(log_controller: &colosseum::logging::LogController) -> Box<dyn colosseum::Scene> {
        let logger = log_controller.logger("cube");
        colosseum::info!(logger, "Starting cube scene");
        Box::new(Scene(logger))
    }
}

impl colosseum::Scene for Scene {
    fn update(&mut self) -> Option<Box<dyn colosseum::Scene>> {
        None
    }

    fn render(&mut self) {}
}
