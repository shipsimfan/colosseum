use crate::{
    graphics::{
        context::Lights,
        lights::{SpotLight, SpotLightInner},
    },
    math::{Color3f, Vector3f},
};
use std::{cell::RefCell, rc::Rc};

impl SpotLight {
    /// Create a new [`SpotLight`]
    pub(in crate::graphics) fn new(
        lights: &mut Lights,
        position: Vector3f,
        distance: f32,
        direction: Vector3f,
        cut_off: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        let spot_light = Rc::new(RefCell::new(SpotLightInner::new(
            position, distance, direction, cut_off, color, brightness,
        )));

        let spot_light_list = lights.push_spot_light(spot_light.clone());

        SpotLight {
            spot_light_list,
            spot_light,
        }
    }
}
