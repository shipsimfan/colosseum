use crate::{
    Result,
    render::{FrameIndex, GpuPointLight, GpuPointLightMatrix, HostDataBuffer, PointLight},
};
use alexandria::math::{Matrix4x4f, Vector3f};
use colosseum_ecs::{Query, Res, ResMut};
use std::f32::consts::PI;

impl PointLight {
    /// The system to copy point light data into the host buffer
    pub(in crate::render) fn system(
        frame_index: Res<FrameIndex>,
        mut light_buffer: ResMut<HostDataBuffer<GpuPointLight>>,
        mut matrix_buffer: ResMut<HostDataBuffer<GpuPointLightMatrix>>,
        point_lights: Query<&PointLight>,
    ) -> Result<()> {
        let light_buffer = &mut light_buffer[&frame_index];
        let matrix_buffer = &mut matrix_buffer[&frame_index];

        light_buffer.reset();
        matrix_buffer.reset();

        let len = point_lights.len();
        light_buffer.reserve(len)?;
        matrix_buffer.reserve(len)?;

        for point_light in point_lights {
            light_buffer.push(point_light.gpu());
            matrix_buffer.push(point_light.view_projections());
        }

        Ok(())
    }

    /// Create a new [`GpuPointLight`] for this point light
    fn gpu(&self) -> GpuPointLight {
        GpuPointLight::new(self.color.with_alpha(1.0), self.position, self.range)
    }

    /// Create a new [`GpuPointLightMatrix`] for this point light
    fn view_projections(&self) -> GpuPointLightMatrix {
        let projection = Matrix4x4f::new_perspective(1.0, PI / 2.0, 0.01, self.range);

        GpuPointLightMatrix::new([
            projection
                * Matrix4x4f::new_look_at(self.position, self.position + Vector3f::X, Vector3f::Y),
            projection
                * Matrix4x4f::new_look_at(self.position, self.position - Vector3f::X, Vector3f::Y),
            projection
                * Matrix4x4f::new_look_at(self.position, self.position + Vector3f::Y, -Vector3f::Z),
            projection
                * Matrix4x4f::new_look_at(self.position, self.position - Vector3f::Y, Vector3f::Z),
            projection
                * Matrix4x4f::new_look_at(self.position, self.position + Vector3f::Z, Vector3f::Y),
            projection
                * Matrix4x4f::new_look_at(self.position, self.position - Vector3f::Z, Vector3f::Y),
        ])
    }
}
