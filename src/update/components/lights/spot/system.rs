use crate::{
    render::{RenderData, RenderSpotLight},
    system_with_extra_data,
    update::{components::SpotLight, ecs::System},
};

impl SpotLight {
    /// Create a system that operates on the [`SpotLight`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) =
            system_with_extra_data!(|render_data: RenderData, spot_lights: SpotLight| {
                render_data.reserve_spot_lights(spot_lights.len()).unwrap();

                for light in spot_lights {
                    render_data.lighting_mut().add_spot_light(RenderSpotLight {
                        color: (light.color * light.intensity).with_alpha(1.0),
                        position: light.position,
                        range: light.range,
                        direction: light.direction,
                        cutoff_angle: light.cutoff_angle,
                        falloff_angle: light.falloff_angle,
                    });
                }
            });
        System::new(type_ids, system)
    }
}
