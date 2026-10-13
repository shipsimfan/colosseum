use crate::update::components::CameraProjection;
use alexandria::math::Vector3f;

impl CameraProjection {
    /// Get the eight corners that make up this projection
    pub(in crate::update) fn shadow_corners(
        &self,
        aspect: f32,
        lambda: f32,
    ) -> ([([Vector3f; 4], f32); 5], [f32; 4]) {
        match self {
            &CameraProjection::Perspective { fov_y, near, far } => {
                perspective_corners(aspect, fov_y, near, far, lambda)
            }
            &CameraProjection::InfinitePerspective {
                fov_y,
                near,
                shadow_distance,
            } => perspective_corners(aspect, fov_y, near, shadow_distance, lambda),
            &CameraProjection::Orthographic { size, near, far } => {
                orthographic_corners(aspect, size, near, far, lambda)
            }
        }
    }
}

fn find_far_plane(near: f32, far: f32, index: u32, lambda: f32) -> f32 {
    let index = index as f32 / 4.0;

    let log = near * (far / near).powf(index);
    let linear = near + (far - near) * index;

    lambda * log + (1.0 - lambda) * linear
}

fn find_far_planes(near: f32, far: f32, lambda: f32) -> [f32; 3] {
    [
        find_far_plane(near, far, 1, lambda),
        find_far_plane(near, far, 2, lambda),
        find_far_plane(near, far, 3, lambda),
    ]
}

fn perspective_corners(
    aspect: f32,
    fov_y: f32,
    near: f32,
    far: f32,
    lambda: f32,
) -> ([([Vector3f; 4], f32); 5], [f32; 4]) {
    let tan_fov = (fov_y / 2.0).tan();
    let far_planes = find_far_planes(near, far, lambda);

    (
        [
            perspective_plane_corners(aspect, tan_fov, near, near),
            perspective_plane_corners(aspect, tan_fov, near, far_planes[0]),
            perspective_plane_corners(aspect, tan_fov, near, far_planes[1]),
            perspective_plane_corners(aspect, tan_fov, near, far_planes[2]),
            perspective_plane_corners(aspect, tan_fov, near, far),
        ],
        [far_planes[0], far_planes[1], far_planes[2], far],
    )
}

fn perspective_plane_corners(
    aspect: f32,
    tan_fov: f32,
    near: f32,
    far: f32,
) -> ([Vector3f; 4], f32) {
    let half_width = (far - near) * tan_fov;
    let half_height = half_width * aspect;
    let depth = (half_width * half_width + half_height * half_height).sqrt();

    let corner = perspective_corner(aspect, tan_fov, far);
    (
        [
            corner,
            Vector3f::new(-corner.x, corner.y, corner.z),
            Vector3f::new(corner.x, -corner.y, corner.z),
            Vector3f::new(-corner.x, -corner.y, corner.z),
        ],
        depth,
    )
}

fn perspective_corner(aspect: f32, tan_fov: f32, depth: f32) -> Vector3f {
    Vector3f::new(aspect * tan_fov * depth, tan_fov * depth, depth)
}

fn orthographic_corners(
    aspect: f32,
    size: f32,
    near: f32,
    far: f32,
    lambda: f32,
) -> ([([Vector3f; 4], f32); 5], [f32; 4]) {
    let half_width = size * aspect;
    let far_planes = find_far_planes(near, far, lambda);

    (
        [
            orthographic_plane_corners(half_width, size, near, near),
            orthographic_plane_corners(half_width, size, near, far_planes[0]),
            orthographic_plane_corners(half_width, size, near, far_planes[1]),
            orthographic_plane_corners(half_width, size, near, far_planes[2]),
            orthographic_plane_corners(half_width, size, near, far),
        ],
        [far_planes[0], far_planes[1], far_planes[2], far],
    )
}

fn orthographic_plane_corners(
    half_width: f32,
    half_height: f32,
    near: f32,
    far: f32,
) -> ([Vector3f; 4], f32) {
    (
        [
            Vector3f::new(half_width, half_height, far),
            Vector3f::new(-half_width, half_height, far),
            Vector3f::new(half_width, -half_height, far),
            Vector3f::new(-half_width, -half_height, far),
        ],
        far - near,
    )
}
