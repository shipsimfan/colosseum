use crate::{
    render::{MaterialKind, ObjectData, RenderData},
    system_with_extra_data_and_setup,
    update::{
        components::{Renderer, Transform},
        ecs::System,
    },
};

impl Renderer {
    /// Create a system that operates on the [`Renderer`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) = system_with_extra_data_and_setup!(
            |entity_count, render_data: RenderData| {
                render_data.reserve_renderables(entity_count).unwrap();
            },
            |render_data, renderer: Renderer, transform: Transform| {
                for (renderer, transform) in renderer.iter().zip(transform) {
                    let object = ObjectData::new(transform.matrix());

                    match renderer.material.kind() {
                        MaterialKind::UnlitOpaque => render_data.add_unlit_opaque_renderable(
                            renderer.material.id(),
                            renderer.mesh,
                            object,
                        ),
                        MaterialKind::LitOpaque => render_data.add_lit_opaque_renderable(
                            renderer.material.id(),
                            renderer.mesh,
                            object,
                        ),
                    }
                }
            }
        );
        System::new(type_ids, system)
    }
}
