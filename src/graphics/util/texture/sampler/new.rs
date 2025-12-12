use crate::{
    Error, Result,
    graphics::{TextureEdge, TextureFilter, util::TextureSampler},
};
use win32::{
    ComPtr,
    d3d11::{D3D11_SAMPLER_DESC, ID3D11Device},
    try_hresult,
};

impl TextureSampler {
    /// Create a new [`TextureSampler`]
    pub fn new(filter: TextureFilter, edge: TextureEdge, device: &ID3D11Device) -> Result<Self> {
        let max_anisotropy = filter.max_anisotropy();
        let filter = filter.to_d3d();

        let border_color = edge.border_color();
        let edge = edge.to_d3d();

        let desc = D3D11_SAMPLER_DESC {
            filter,
            address_u: edge,
            address_v: edge,
            address_w: edge,
            max_anisotropy,
            border_color,
            ..Default::default()
        };
        let sampler =
            ComPtr::new_in(|sampler| try_hresult!(device.create_sampler_state(&desc, sampler)))
                .map_err(|error| Error::new_inner("unable to create sampler state", error))?;

        Ok(TextureSampler { sampler })
    }
}
