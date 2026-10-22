use crate::{
    Result,
    render::{
        DescriptorSetBinding, DescriptorSetLayouts, GraphicsDevice, MAX_FRAMES_IN_FLIGHT,
        frame_graph::{
            FrameGraphResources, FrameGraphTransientResourceInfo,
            resources::FrameGraphTransientResourceBuffer,
        },
    },
};
use alexandria::gpu::{VulkanDescriptorPool, VulkanDescriptorPoolSize, VulkanDescriptorSet};
use colosseum_core::new_error;
use std::{any::TypeId, borrow::Cow};
#[cfg(debug_assertions)]
use std::{ffi::CString, str::FromStr};

impl FrameGraphResources {
    /// Create a new instance of [`FrameGraphResources`]
    pub fn new(
        native_scale_resources: Vec<FrameGraphTransientResourceInfo>,
        render_scale_resources: Vec<FrameGraphTransientResourceInfo>,

        mut descriptors: Vec<VulkanDescriptorPoolSize>,
        descriptor_sets: Vec<(TypeId, Cow<'static, str>, &'static [DescriptorSetBinding])>,
        descriptor_set_layouts: &DescriptorSetLayouts,

        device: &GraphicsDevice,
    ) -> Result<FrameGraphResources> {
        // Create descriptor pool
        for pool_size in &mut descriptors {
            *pool_size.count_mut() *= MAX_FRAMES_IN_FLIGHT as u32;
        }

        let mut descriptor_pool = device
            .create_descriptor_pool(
                0,
                (descriptor_sets.len() * MAX_FRAMES_IN_FLIGHT) as _,
                &descriptors,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create the frame graph descriptor pool - {}",
                    error
                )
            })?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut descriptor_pool, c"Frame Graph Descriptor Pool")
            .map_err(|error| {
                new_error!("unable to set frame graph descriptor pool name - {}", error)
            })?;

        // Allocate descriptor sets
        let mut new_descriptor_sets = Vec::with_capacity(descriptor_sets.len());
        for (layout, name, bindings) in descriptor_sets {
            new_descriptor_sets.push((
                allocate_descriptor_sets(
                    &mut descriptor_pool,
                    layout,
                    &name,
                    descriptor_set_layouts,
                    device,
                )?,
                bindings,
            ));
        }

        Ok(FrameGraphResources {
            external_resources: Vec::new(),
            native_scale: FrameGraphTransientResourceBuffer::new(
                "Native Scale Memory",
                native_scale_resources,
            ),
            render_scale: FrameGraphTransientResourceBuffer::new(
                "Render Scale Memory",
                render_scale_resources,
            ),

            descriptor_pool: Some(descriptor_pool),
            descriptor_sets: new_descriptor_sets,
        })
    }

    /// Create an empty instance of [`FrameGraphResources`]
    pub fn empty() -> FrameGraphResources {
        FrameGraphResources {
            external_resources: Vec::new(),
            native_scale: FrameGraphTransientResourceBuffer::new("", Vec::new()),
            render_scale: FrameGraphTransientResourceBuffer::new("", Vec::new()),

            descriptor_pool: None,
            descriptor_sets: Vec::new(),
        }
    }
}

/// Allocate an array of descriptor sets for the given layout, one for each frame in flight
fn allocate_descriptor_sets(
    descriptor_pool: &mut VulkanDescriptorPool,
    layout: TypeId,
    name: &str,
    descriptor_set_layouts: &DescriptorSetLayouts,
    device: &GraphicsDevice,
) -> Result<[VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT]> {
    Ok([
        allocate_descriptor_set(
            descriptor_pool,
            layout,
            name,
            descriptor_set_layouts,
            0,
            device,
        )?,
        allocate_descriptor_set(
            descriptor_pool,
            layout,
            name,
            descriptor_set_layouts,
            1,
            device,
        )?,
        allocate_descriptor_set(
            descriptor_pool,
            layout,
            name,
            descriptor_set_layouts,
            2,
            device,
        )?,
    ])
}

/// Allocate a single descriptor set for the given layout from the descriptor pool
fn allocate_descriptor_set(
    descriptor_pool: &mut VulkanDescriptorPool,
    layout: TypeId,
    name: &str,
    descriptor_set_layouts: &DescriptorSetLayouts,
    index: usize,
    device: &GraphicsDevice,
) -> Result<VulkanDescriptorSet> {
    let name = format!("{} {}", name, index);

    #[allow(unused_mut)]
    let mut descriptor_set = descriptor_pool
        .allocate_descriptor_set(descriptor_set_layouts.get_by_id(layout))
        .map_err(|error| new_error!("unable to allocate \"{}\" - {}", name, error))?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(&mut descriptor_set, &CString::from_str(&name).unwrap())
        .map_err(|error| new_error!("unable to set name for \"{}\" - {}", name, error))?;

    Ok(descriptor_set)
}
