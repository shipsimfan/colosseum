#[allow(unused)]
struct Scene(colosseum::logging::Logger);

fn main() {
    colosseum::run("Cube", None, None, Scene::new);
}

impl Scene {
    pub fn new(context: colosseum::UpdateContext) -> Box<dyn colosseum::Scene> {
        let logger = context.log_controller().logger("cube");
        colosseum::info!(logger, "Starting cube scene");
        Box::new(Scene(logger))
    }
}

impl colosseum::Scene for Scene {
    fn update(&mut self, _: colosseum::UpdateContext) -> Option<Box<dyn colosseum::Scene>> {
        None
    }

    fn render(&mut self, _: colosseum::RenderContext) {}
}
