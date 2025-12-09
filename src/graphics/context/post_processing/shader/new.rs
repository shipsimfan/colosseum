use crate::{
    Result,
    graphics::{PostProcessingShader, ShaderSource, util::PixelShader},
};
use win32::d3d11::ID3D11Device;

impl PostProcessingShader {
    /// Create a new [`PostProcessingShader`]
    pub(in crate::graphics::context::post_processing) fn new(
        source: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        assert_eq!(source.r#type(), "ps_5_0");

        let pixel_shader = PixelShader::new(source.content(), device)?;

        Ok(PostProcessingShader { pixel_shader })
    }
}
