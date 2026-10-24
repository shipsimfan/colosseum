use crate::{
    Result,
    render::{Assets, Mesh, Primitives, RenderGpuTransferQueue, Vertex},
};
use alexandria::{Id, gpu::VulkanQueue};

mod cube;
mod cylinder;
mod plane;
mod quad;
mod sphere;

impl Primitives {
    /// Create a new set of [`Primitives`]
    pub(in crate::render::public_data) fn new(
        assets: &mut Assets,
        mut gpu_transfer_queue: Option<(&mut RenderGpuTransferQueue, &mut VulkanQueue)>,
    ) -> Result<Primitives> {
        let quad = create_mesh_primitive(
            "Quad Primitive",
            quad::quad,
            assets,
            &mut gpu_transfer_queue,
        )?;
        let plane = create_mesh_primitive(
            "Plane Primitive",
            plane::plane,
            assets,
            &mut gpu_transfer_queue,
        )?;
        let cube = create_mesh_primitive(
            "Cube Primitive",
            cube::cube,
            assets,
            &mut gpu_transfer_queue,
        )?;
        let sphere = create_mesh_primitive(
            "Sphere Primitive",
            sphere::sphere,
            assets,
            &mut gpu_transfer_queue,
        )?;
        let cylinder = create_mesh_primitive(
            "Cylinder Primitive",
            cylinder::cylinder,
            assets,
            &mut gpu_transfer_queue,
        )?;

        Ok(Primitives {
            quad,
            plane,
            cube,
            sphere,
            cylinder,
        })
    }
}

/// Create a mesh primitive
fn create_mesh_primitive(
    name: &str,
    f: fn() -> (Vec<Vertex>, Vec<u32>),
    assets: &mut Assets,
    gpu_transfer_queue: &mut Option<(&mut RenderGpuTransferQueue, &mut VulkanQueue)>,
) -> Result<Id<Mesh>> {
    // Start the transfer
    let (vertices, indices) = f();
    let loading_mesh = assets.create_mesh(name, vertices, indices)?;

    // Wait for the transfer to complete
    match gpu_transfer_queue {
        Some((transfer_queue, queue)) => {
            while !loading_mesh.is_complete() {
                transfer_queue.handle_command(queue, true)?;
            }
        }
        None => {
            loading_mesh.wait()?;
        }
    }

    // Insert into the assets
    Ok(assets.complete_mesh(loading_mesh))
}
