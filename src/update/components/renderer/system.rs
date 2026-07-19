use crate::{
    render::{MaterialKind, RenderData},
    system_with_extra_data,
    update::{components::Renderer, ecs::System},
};

impl Renderer {
    /// Create a system that operates on the [`Renderer`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) =
            system_with_extra_data!(|render_data: RenderData, renderer: Renderer| {
                for renderer in renderer {
                    match renderer.material.kind() {
                        MaterialKind::UnlitOpaque => render_data
                            .add_unlit_opaque_renderable(renderer.material.id(), renderer.mesh),
                    }
                }
            });
        System::new(type_ids, system)
    }
}
