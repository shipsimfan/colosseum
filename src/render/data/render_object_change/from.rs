use crate::render::{MaterialId, MaterialKind, RenderMaterial, RenderObjectChange};

impl From<(MaterialKind, RenderMaterial)> for RenderObjectChange {
    fn from((kind, material): (MaterialKind, RenderMaterial)) -> Self {
        RenderObjectChange::AddMaterial { kind, material }
    }
}

impl From<MaterialId> for RenderObjectChange {
    fn from(material: MaterialId) -> Self {
        RenderObjectChange::RemoveMaterial { material }
    }
}
