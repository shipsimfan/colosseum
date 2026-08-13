mod default;
mod matrix;

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
    },

    /// The orthographic projection, which keeps parallel lines parallel and does not simulate
    /// perspective
    Orthographic {
        /// The left clipping plane distance
        left: f32,

        /// The right clipping plane distance
        right: f32,

        /// The bottom clipping plane distance
        bottom: f32,

        /// The top clipping plane distance
        top: f32,

        /// The near clipping plane distance
        near: f32,

        /// The far clipping plane distance
        far: f32,
    },
}
