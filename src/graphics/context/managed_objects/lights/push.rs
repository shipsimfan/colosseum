use crate::graphics::{context::Lights, lights::DirectionalLightInner};
use std::{cell::RefCell, rc::Rc};

impl Lights {
    /// Push a new directional `light` to the active list, returning the shared list
    pub fn push_directional_light(
        &mut self,
        light: Rc<RefCell<DirectionalLightInner>>,
    ) -> Rc<RefCell<(Vec<Rc<RefCell<DirectionalLightInner>>>, bool)>> {
        self.directional_lights.push(light)
    }
}
