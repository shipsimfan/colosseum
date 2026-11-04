use crate::{
    Result,
    render::{FrameIndex, GpuSpotLight, GpuSpotLightMatrix, HostDataBuffer, SpotLight},
};
use alexandria::math::{Matrix4x4f, Vector3f};
use colosseum_ecs::{Query, Res, ResMut};

impl SpotLight {
    /// The system to copy spotlight data into the host buffer
    pub(in crate::render) fn system(
        frame_index: Res<FrameIndex>,
        mut light_buffer: ResMut<HostDataBuffer<GpuSpotLight>>,
        mut matrix_buffer: ResMut<HostDataBuffer<GpuSpotLightMatrix>>,
        spot_lights: Query<&SpotLight>,
    ) -> Result<()> {
        let light_buffer = &mut light_buffer[&frame_index];
        let matrix_buffer = &mut matrix_buffer[&frame_index];

        light_buffer.reset();
        matrix_buffer.reset();

        let len = spot_lights.len();
        light_buffer.reserve(len)?;
        matrix_buffer.reserve(len)?;

        for spot_light in spot_lights {
            light_buffer.push(spot_light.gpu());
            matrix_buffer.push(spot_light.view_projection());
        }

        Ok(())
    }

    /// Create a new [`GpuSpotLight`] for this spotlight
    fn gpu(&self) -> GpuSpotLight {
        GpuSpotLight::new(
            self.color.with_alpha(1.0),
            self.position,
            self.range,
            self.direction,
            self.cutoff_angle,
            self.falloff_angle,
        )
    }

    /// Create a new [`GpuSpotLightMatrix`] for this spotlight
    fn view_projection(&self) -> GpuSpotLightMatrix {
        let projection = Matrix4x4f::new_perspective(1.0, self.cutoff_angle, 0.01, self.range);

        let mut up = Vector3f::Y;
        if up.dot(self.direction).abs() > 0.999 {
            up = Vector3f::X;
        }
        let view = Matrix4x4f::new_look_at(self.position, self.position + self.direction, up);

        GpuSpotLightMatrix::new(projection * view)
    }
}
