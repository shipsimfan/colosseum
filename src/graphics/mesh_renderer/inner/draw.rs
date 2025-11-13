use crate::graphics::MeshRendererInner;
use win32::d3d11::ID3D11DeviceContext;

impl MeshRendererInner {
    /// Draw this mesh using the active settings
    pub(in crate::graphics) fn draw(&mut self, device_context: &mut ID3D11DeviceContext) {
        self.mesh.bind(device_context);
        device_context.draw_indexed(self.mesh.index_count(), 0, 0);
    }
}
