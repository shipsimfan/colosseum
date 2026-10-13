use crate::{
    render::{MaterialId, MaterialKind, Mesh, RenderMaterial, RenderMesh, RenderObjectChange},
    update::GpuAllocatedMemory,
};
use alexandria::{
    Id,
    math::{Color4f, Linear},
};

impl From<(MaterialKind, RenderMaterial)> for RenderObjectChange {
    fn from((kind, material): (MaterialKind, RenderMaterial)) -> Self {
        RenderObjectChange::AddMaterial { kind, material }
    }
}

impl From<(MaterialId, Color4f<Linear>)> for RenderObjectChange {
    fn from((material, color): (MaterialId, Color4f<Linear>)) -> Self {
        RenderObjectChange::ChangeMaterialColor { material, color }
    }
}

impl From<MaterialId> for RenderObjectChange {
    fn from(material: MaterialId) -> Self {
        RenderObjectChange::RemoveMaterial { material }
    }
}

impl From<RenderMesh> for RenderObjectChange {
    fn from(mesh: RenderMesh) -> Self {
        RenderObjectChange::AddMesh { mesh }
    }
}

impl From<(Id<Mesh>, GpuAllocatedMemory)> for RenderObjectChange {
    fn from((mesh, memory): (Id<Mesh>, GpuAllocatedMemory)) -> Self {
        RenderObjectChange::RemoveMesh { mesh, memory }
    }
}
