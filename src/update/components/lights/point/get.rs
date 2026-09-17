use std::f32::consts::PI;

use crate::update::components::PointLight;
use alexandria::math::{Color3f, Linear, Matrix4x4f, Vector3f};

impl PointLight {
    /// Get the color of the point light
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }

    /// Get the intensity of the point light
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get the position of the point light
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the range of the point light
    pub fn range(&self) -> f32 {
        self.range
    }

    /// Get the view-projections of this light
    pub(in crate::update::components::lights::point) fn view_projections(
        &mut self,
    ) -> [Matrix4x4f; 6] {
        if self.dirty {
            let projection = Matrix4x4f::new_perspective(1.0, PI / 2.0, 0.01, self.range);

            self.view_projections = [
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position + Vector3f::X,
                        Vector3f::Y,
                    ),
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position - Vector3f::X,
                        Vector3f::Y,
                    ),
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position + Vector3f::Y,
                        -Vector3f::Z,
                    ),
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position - Vector3f::Y,
                        Vector3f::Z,
                    ),
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position + Vector3f::Z,
                        Vector3f::Y,
                    ),
                projection
                    * Matrix4x4f::new_look_at(
                        self.position,
                        self.position - Vector3f::Z,
                        Vector3f::Y,
                    ),
            ];

            self.dirty = false;
        }

        self.view_projections
    }
}
