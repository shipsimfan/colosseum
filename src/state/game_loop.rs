use super::Colosseum;
use crate::Scene;

impl Colosseum {
    pub(crate) fn game_loop(&mut self, mut scene: Box<dyn Scene>) {
        scene.on_start(self.update_context());

        while self.graphics.window().poll_events() {
            let next_scene = scene.update(self.update_context());
            scene.render(self.render_context());

            if let Some(next_scene) = next_scene {
                scene.on_finish(self.update_context());
                scene = next_scene;
                scene.on_start(self.update_context());
            }
        }

        scene.on_finish(self.update_context());
    }
}
