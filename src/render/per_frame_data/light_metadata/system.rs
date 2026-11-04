use crate::render::{
    AmbientLight, DirectionalLight, FrameIndex, HostDataBuffer, LightMetadata, PointLight,
    SpotLight,
};
use colosseum_ecs::{Query, Res, ResMut};

impl LightMetadata {
    /// The system for updating light metadata
    pub fn system(
        mut buffer: ResMut<HostDataBuffer<Self>>,
        frame_index: Res<FrameIndex>,
        ambient_light: Res<AmbientLight>,
        directional_lights: Query<&DirectionalLight>,
        point_lights: Query<&PointLight>,
        spot_lights: Query<&SpotLight>,
    ) {
        let metadata = LightMetadata {
            ambient_light: ambient_light.color().with_alpha(1.0),
            num_directional_lights: directional_lights.len() as _,
            num_point_lights: point_lights.len() as _,
            num_spot_lights: spot_lights.len() as _,
        };

        buffer[&frame_index][0] = metadata;
    }
}
