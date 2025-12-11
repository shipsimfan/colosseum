use crate::graphics::util::ConstantBuffer;

impl ConstantBuffer<()> {
    /// The slot to put the frame-wide constant buffer in
    pub const FRAME_SLOT: u32 = 0;

    /// The slot to put the camera constant buffer in
    pub const CAMERA_SLOT: u32 = 1;

    /// The slot to put the material constant buffer in
    pub const MATERIAL_SLOT: u32 = 2;

    /// The slot to put the lighting constant buffer in
    pub const LIGHTING_SLOT: u32 = 3;
}
