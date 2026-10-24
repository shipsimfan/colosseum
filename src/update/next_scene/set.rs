use crate::update::NextScene;

impl NextScene {
    /// Set the next scene initialization function
    pub fn set<Scene: crate::update::Scene>(&mut self, next_scene: Scene) {
        self.next_scene = Some(Box::new(next_scene));
    }
}
