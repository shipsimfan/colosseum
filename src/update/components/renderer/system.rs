use crate::{
    render::{MaterialKind, ObjectData, RenderData},
    system_with_extra_data,
    update::{
        components::{Renderer, Transform},
        ecs::System,
    },
};

impl Renderer {
    /// Create a system that operates on the [`Renderer`] component
    pub(in crate::update) fn system() -> System<RenderData> {
        let (type_ids, system) =
            system_with_extra_data!(|render_data: RenderData,
                                     renderer: Renderer,
                                     transform: Transform| {
                render_data.reserve_renderables(renderer.len()).unwrap();

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
            });
        System::new(type_ids, system)
    }
}
