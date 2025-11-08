use crate::{
    Error, Result,
    graphics::{Output, OutputResolution},
    math::Vector2,
};
use win32::{
    ComPtr,
    dxgi::{DXGI_OUTPUT_DESC, IDXGIAdapter1, IDXGIOutput},
    dxgi1_2::IDXGIOutput1,
    try_hresult,
};

impl Output {
    /// Create a new [`Output`]
    pub(in crate::graphics::adapter::output) fn new(
        mut output: ComPtr<IDXGIOutput>,
        adapter: ComPtr<IDXGIAdapter1>,
    ) -> Result<Self> {
        let mut output = output
            .query_interface::<IDXGIOutput1>()
            .map_err(|os| Error::new_inner("unable to get modern adapter interface", os))?;

        // Get the description
        let mut desc = DXGI_OUTPUT_DESC::default();
        try_hresult!(output.get_desc(&mut desc))
            .map_err(|os| Error::new_inner("unable to get adapter description", os))?;

        // Get resolutions
        let resolutions = OutputResolution::enumerate(&mut output)?;

        // Extract position
        let position = Vector2::new(desc.desktop_coordinates.left, desc.desktop_coordinates.top);

        // Extract name
        let mut name_length = desc.device_name.len();
        for i in 0..desc.device_name.len() {
            if desc.device_name[i] == 0 {
                name_length = i;
                break;
            }
        }
        let name = String::from_utf16_lossy(&desc.device_name[..name_length]);

        Ok(Output {
            name,
            position,
            resolutions,
            output,
            adapter,
        })
    }
}
