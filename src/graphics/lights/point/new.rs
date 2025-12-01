use crate::{
    graphics::{
        context::Lights,
        lights::{PointLight, PointLightInner},
    },
    math::{Color3f, Vector3f},
};
use std::{cell::RefCell, rc::Rc};

impl PointLight {
    /// Create a new [`PointLight`]
    pub(in crate::graphics) fn new(
        lights: &mut Lights,
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        let point_light = Rc::new(RefCell::new(PointLightInner::new(
            position, radius, color, brightness,
        )));

        let point_light_list = lights.push_point_light(point_light.clone());

        PointLight {
            point_light_list,
            point_light,
        }
    }
}
