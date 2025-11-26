use std::borrow::Cow;

use crate::{
    graphics::{Mesh, MeshPrimitives},
    math::{Vector2f, Vector3f},
};

impl MeshPrimitives {
    /// Creates a quad facing `normal` with `size`
    pub fn quad(normal: Vector3f, size: Vector2f) -> Mesh {
        let normal = normal.normalized();

        // Select an axis not aligned with `normal`
        let axis = if normal.x.abs() < 0.9 {
            Vector3f::UNIT_X
        } else {
            Vector3f::UNIT_Y
        };

        // Build basis
        let tangent = normal.cross(axis).normalized();
        let bitangent = normal.cross(tangent);

        let half_w = size.x * 0.5;
        let half_h = size.y * 0.5;

        let vertices = vec![
            -tangent * half_w - bitangent * half_h,
            tangent * half_w - bitangent * half_h,
            tangent * half_w + bitangent * half_h,
            -tangent * half_w + bitangent * half_h,
        ];

        unsafe { Mesh::new_unchecked(Cow::Owned(vertices), &[]) }
    }
}
