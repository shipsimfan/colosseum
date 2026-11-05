use crate::{
    Result,
    render::{
        FrameIndex, GpuRenderable, HostDataBuffer, LitForwardMaterial, MaterialId, Renderables,
        Renderer, UnlitMaterial,
    },
    update::Transform,
};
use colosseum_ecs::{Query, Res, ResMut};

impl Renderer {
    /// Setup the renderable data and list for the current frame
    pub(in crate::render) fn system(
        query: Query<(&mut Renderer, &Transform)>,
        frame_index: Res<FrameIndex>,
        mut renderable_buffer: ResMut<HostDataBuffer<GpuRenderable>>,
        mut unlit_renderables: ResMut<Renderables<UnlitMaterial>>,
        mut lit_forward_renderables: ResMut<Renderables<LitForwardMaterial>>,
    ) -> Result<()> {
        let renderable_buffer = &mut renderable_buffer[&frame_index];

        unlit_renderables.reset();
        lit_forward_renderables.reset();

        renderable_buffer.reset();
        renderable_buffer.reserve(query.len())?;

        for (renderer, transform) in query {
            renderable_buffer.push(GpuRenderable::new(transform));

            match renderer.material {
                MaterialId::Unlit(material) => {
                    unlit_renderables.push(material, renderer.mesh, renderable_buffer.count() - 1);
                }
                MaterialId::LitForward(material) => {
                    lit_forward_renderables.push(
                        material,
                        renderer.mesh,
                        renderable_buffer.count() - 1,
                    );
                }
            }
        }

        Ok(())
    }
}
