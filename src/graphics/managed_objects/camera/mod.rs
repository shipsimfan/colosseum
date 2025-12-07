use crate::{Transform, graphics::util::ConstantBuffer, math::Matrix4x4f, util::Handle};
use cb_content::CameraCbContent;
use win32::d3d11::D3D11_VIEWPORT;

mod cameras;
mod cb_content;
mod projection_type;

mod active;
mod bind;
mod look_at;
mod new;
mod position;
mod projection;
mod resize;
mod rotation;
mod viewport;

pub use cameras::Cameras;
pub use projection_type::CameraProjection;

/// A handle pointing to a camera
pub type CameraHandle = Handle<Camera>;

/// A camera which represents a point of view to render from
pub struct Camera {
    /// Is this camera active?
    active: bool,

    /// The location and orientation of the camera
    transform: Transform,

    /// The last epoch of the transform
    transform_epoch: u32,

    /// The current projection the camera uses
    projection: CameraProjection,

    /// Has the projection changed this frame?
    projection_dirty: bool,

    /// The matrix which is the result of `projection`
    projection_matrix: Matrix4x4f,

    /// The buffer holding the final camera matrix for shaders
    buffer: ConstantBuffer<CameraCbContent>,

    /// The region of the window that this camera displays to, in percentages of screen size
    relative_viewport: D3D11_VIEWPORT,

    /// The region of the window that this camera displays to, in pixels
    screen_viewport: D3D11_VIEWPORT,

    /// Has the viewport changed this frame?
    viewport_dirty: bool,
}
