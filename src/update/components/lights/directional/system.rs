use crate::{
    render::{RenderData, RenderDirectionalLight},
    system_with_extra_data,
    update::{components::DirectionalLight, ecs::System},
};

impl DirectionalLight {
    /// Create a system that operates on the [`DirectionalLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) = system_with_extra_data!(
            |render_data: RenderData, directional_lights: DirectionalLight| {
                render_data
                    .reserve_directional_lights(directional_lights.len())
                    .unwrap();

                for light in directional_lights {
                    render_data
                        .lighting_mut()
                        .add_directional_light(RenderDirectionalLight {
                            color: (light.color * light.intensity).with_alpha(1.0),
                            direction: light.direction,
                        });
                }
            }
        );
        System::new(type_ids, system)
    }
}
