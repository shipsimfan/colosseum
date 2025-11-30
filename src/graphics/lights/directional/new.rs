use crate::{
    graphics::{
        context::Lights,
        lights::{DirectionalLight, DirectionalLightInner},
    },
    math::{Color3f, Vector3f},
};
use std::{cell::RefCell, rc::Rc};

impl DirectionalLight {
    /// Create a new [`DirectionalLight`]
    pub(in crate::graphics) fn new(
        lights: &mut Lights,
        direction: Vector3f,
        color: Color3f,
    ) -> Self {
        let directional_light = Rc::new(RefCell::new(DirectionalLightInner::new(direction, color)));

        let directional_light_list = lights.push_directional_light(directional_light.clone());

        DirectionalLight {
            directional_light_list,
            directional_light,
        }
    }
}
