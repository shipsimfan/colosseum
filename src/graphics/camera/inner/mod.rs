use crate::{
    graphics::{CameraProjection, Transform},
    math::Matrix4x4f,
};
use win32::{
    ComPtr,
    d3d11::{D3D11_VIEWPORT, ID3D11Buffer},
};

mod active;
mod bind;
mod new;
mod position;
mod projection;
mod resize;
mod rotation;
mod viewport;

/// The actual definition of a camera
pub struct CameraInner {
    /// Is this camera active?
    active: bool,

    /// The location and orientation of the camera
    transform: Transform,

    /// The current projection the camera uses
    projection: CameraProjection,

    /// Has the projection changed this frame?
    projection_dirty: bool,

    /// The matrix which is the result of `projection`
    projection_matrix: Matrix4x4f,

    /// The constant buffer holding the combined world-projection matrix
    buffer: ComPtr<ID3D11Buffer>,

    /// The region of the window that this camera displays to, in percentages of screen size
    relative_viewport: D3D11_VIEWPORT,

    /// The region of the window that this camera displays to, in pixels
    screen_viewport: D3D11_VIEWPORT,

    /// Has the viewport changed this frame?
    viewport_dirty: bool,
}
