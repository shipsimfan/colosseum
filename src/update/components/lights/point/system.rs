use crate::{
    render::{RenderData, RenderPointLight},
    system_with_extra_data,
    update::{components::PointLight, ecs::System},
};

impl PointLight {
    /// Create a system that operates on the [`PointLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) =
            system_with_extra_data!(|render_data: RenderData, point_lights: PointLight| {
                render_data
                    .reserve_point_lights(point_lights.len())
                    .unwrap();

                for light in point_lights {
                    render_data
                        .lighting_mut()
                        .add_point_light(RenderPointLight {
                            color: (light.color * light.intensity).with_alpha(1.0),
                            position: light.position,
                            range: light.range,
                        });
                }
            });
        System::new(type_ids, system)
    }
}
