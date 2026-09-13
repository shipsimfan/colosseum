mod default;
mod depth;
mod matrix;
mod shadow_corners;

/// A projection for a camera, which defines how 3D points are projected onto the 2D screen
pub enum CameraProjection {
    /// The perspective projection, which simulates the way the human eye sees the world
    Perspective {
        /// The field of view in the y direction, in degrees
        fov_y: f32,

        /// The near clipping plane distance
        near: f32,

        /// The far clipping plane distance
        far: f32,
    },

    /// The infinite perspective projection, which is like the perspective projection but with an
    /// infinite far clipping plane
    InfinitePerspective {
        /// The field of view in the y direction, in degrees
        fov_y: f32,

        /// The near clipping plane distance
        near: f32,

        /// The distance at which directional light shadows are rendered
        shadow_distance: f32,
    },

    /// The orthographic projection, which keeps parallel lines parallel and does not simulate
    /// perspective
    Orthographic {
        /// The half-height of the orthographic view volume
        ///
        /// The width is determined by the aspect ratio of the camera
        size: f32,

        /// The near clipping plane distance
        near: f32,

        /// The far clipping plane distance
        far: f32,
    },
}
