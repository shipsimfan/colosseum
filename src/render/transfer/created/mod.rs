use crate::render::RenderMesh;

/// A created object being sent from the transfer thread to the render job
pub(in crate::render) enum CreatedRenderObject {
    /// A mesh that has been created and is ready to be used by the render job
    Mesh(RenderMesh),
}
