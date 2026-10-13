use crate::{
    render::{RenderData, RenderPointLight},
    system_with_extra_data_and_setup,
    update::{components::PointLight, ecs::System},
};

impl PointLight {
    /// Create a system that operates on the [`PointLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) = system_with_extra_data_and_setup!(
            |entity_count, render_data: RenderData| {
                render_data.reserve_point_lights(entity_count).unwrap();
            },
            |render_data, point_lights: PointLight| {
                for light in point_lights {
                    render_data.lighting_mut().add_point_light(
                        RenderPointLight {
                            color: (light.color * light.intensity).with_alpha(1.0),
                            position: light.position,
                            range: light.range,
                        },
                        light.view_projections(),
                    );
                }
            }
        );
        System::new(type_ids, system)
    }
}
