use crate::{
    Result,
    render::{GpuTransferQueue, Mesh, RenderJob, Shader, ShaderId, ShaderKind, Vertex},
    update::{UpdateRenderObjects, render_objects::GpuAllocator},
};
use alexandria::{Id, SlotMap, gpu::compile_shader};
use primitives::*;

mod primitives;

compile_shader! {
    /// The vertex shader code for unlit objects
    const UNLIT_SHADER = "unlit-opaque.slang",
    vert_main,
    frag_main
}

compile_shader! {
    /// The vertex shader code for lit objects
    const LIT_SHADER = "lit-opaque.slang",
    vert_main,
    frag_main
}

impl UpdateRenderObjects {
    /// Create a new set of [`UpdateRenderObjects`]
    pub(in crate::update) fn new(
        transfer_queue: GpuTransferQueue,
        render_job: &mut RenderJob,
    ) -> Result<UpdateRenderObjects> {
        // Extract the necessary data from the render job
        let device = render_job.device().clone();
        let memory_properties = render_job.memory_properties().clone();
        let fixed_render_objects = render_job.fixed_render_objects().clone();

        // Create allocator for GPU meshes
        let mesh_allocator = GpuAllocator::new(
            16 * 1024 * 1024, // 16 MB
            256,              // 256 B
            4 * 1024 * 1024,  // 4 MB
            memory_properties,
            device.clone(),
        );

        // Create default shaders
        let mut unlit_shaders = SlotMap::new();
        let default_unlit_shader = ShaderId::new(
            ShaderKind::Unlit,
            unlit_shaders.insert(Shader::new(&UNLIT_SHADER, &device)?),
        );

        let mut lit_shaders = SlotMap::new();
        let default_lit_shader = ShaderId::new(
            ShaderKind::Lit,
            lit_shaders.insert(Shader::new(&LIT_SHADER, &device)?),
        );

        let mut render_objects = UpdateRenderObjects {
            device,
            fixed_render_objects,
            mesh_allocator,
            meshes: SlotMap::new(),
            unlit_shaders,
            lit_shaders,
            default_unlit_shader,
            default_lit_shader,
            unlit_opaque_materials: SlotMap::new(),
            lit_opaque_materials: SlotMap::new(),
            transfer_queue,
            quad: unsafe { std::mem::zeroed() },
            plane: unsafe { std::mem::zeroed() },
            cube: unsafe { std::mem::zeroed() },
            sphere: unsafe { std::mem::zeroed() },
            cylinder: unsafe { std::mem::zeroed() },
        };

        // Transfer primitive meshes to the GPU
        render_objects.quad = render_objects.transfer_primitive_mesh(
            QUAD_VERTICES.to_vec(),
            QUAD_INDICES.to_vec(),
            render_job,
        )?;
        render_objects.cube = render_objects.transfer_primitive_mesh(
            CUBE_VERTICES.to_vec(),
            CUBE_INDICES.to_vec(),
            render_job,
        )?;
        let (plane_vertices, plane_indices) = primitives::plane();
        render_objects.plane =
            render_objects.transfer_primitive_mesh(plane_vertices, plane_indices, render_job)?;
        let (sphere_vertices, sphere_indices) = primitives::sphere();
        render_objects.sphere =
            render_objects.transfer_primitive_mesh(sphere_vertices, sphere_indices, render_job)?;
        let (cylinder_vertices, cylinder_indices) = primitives::cylinder();
        render_objects.cylinder = render_objects.transfer_primitive_mesh(
            cylinder_vertices,
            cylinder_indices,
            render_job,
        )?;

        Ok(render_objects)
    }

    fn transfer_primitive_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        render_job: &mut RenderJob,
    ) -> Result<Id<Mesh>> {
        let mut transfer = self.create_mesh(vertices, indices)?;

        render_job.wait_for_transfer(&mut transfer)?;

        let (mesh, render_mesh, allocation) = transfer.take();
        render_job
            .render_data()
            .add_render_object_change(render_mesh);
        Ok(self.complete_mesh(mesh, allocation))
    }
}
