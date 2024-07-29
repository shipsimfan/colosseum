use super::Colosseum;
use crate::Scene;

impl Colosseum {
    pub(crate) fn game_loop(&mut self, mut scene: Box<dyn Scene>) {
        while self.window.poll_events() {
            let next_scene = scene.update(self.update_context());
            scene.render(self.render_context());

            if let Some(next_scene) = next_scene {
                scene = next_scene;
            }
        }
    }
}
