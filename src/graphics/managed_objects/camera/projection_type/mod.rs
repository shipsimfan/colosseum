mod matrix;

/// A type of projection that a camera can apply
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraProjection {
    /// Projects using perpsective
    Perspective {
        /// The field of view of the projection
        fov: f32,

        /// The closest an object can be to be rendered
        near: f32,

        /// The furthest an object can be to be rendered
        far: f32,
    },

    /// Projects orthographically
    Orthographic {
        /// Half the height of the projection
        size: f32,

        /// The closest an object can be to be rendered
        near: f32,

        /// The furthest an object can be to be rendered
        far: f32,
    },
}
