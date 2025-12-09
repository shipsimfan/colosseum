#[cfg(debug_assertions)]
use crate::graphics::context::{D3D11InfoQueue, DXGIInfoQueue};
use crate::{
    Error, ManagedObjects, Result,
    graphics::{
        Adapter, GraphicsContext, GraphicsSettings,
        context::{BUFFER_COUNT, PostProcessing, SWAP_CHAIN_FLAGS, SWAPCHAIN_FORMAT},
    },
    info,
    logging::LogController,
    math::Vector2u,
    message_thread::MessageThread,
    warning,
};
use std::{ptr::null_mut, rc::Rc, sync::Arc};
use win32::{
    ComPtr, HWND, TRUE, UINT,
    d3d11::{
        D3D11_BLEND_DESC, D3D11_CREATE_DEVICE_FLAG, D3D11_DEPTH_STENCIL_DESC,
        D3D11_RASTERIZER_DESC, D3D11_SDK_VERSION, D3D11CreateDeviceAndSwapChain,
    },
    d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL},
    dxgi::{
        DXGI_MODE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    },
    try_hresult,
};

const BASE_DEVICE_FLAGS: UINT = D3D11_CREATE_DEVICE_FLAG::BgraSupport as _;

#[cfg(debug_assertions)]
const DEVICE_FLAGS: UINT = BASE_DEVICE_FLAGS | D3D11_CREATE_DEVICE_FLAG::Debug as UINT;
#[cfg(not(debug_assertions))]
const DEVICE_FLAGS: UINT = BASE_DEVICE_FLAGS;

const FEATURE_LEVELS: &[D3D_FEATURE_LEVEL] = &[D3D_FEATURE_LEVEL::_11_0, D3D_FEATURE_LEVEL::_11_1];

impl GraphicsContext {
    /// Creates a new [`GraphicsContext`] given the options
    pub(crate) fn new(
        window: HWND,
        settings: &GraphicsSettings,
        message_thread: Rc<MessageThread>,
        log_controller: &Arc<LogController>,
    ) -> Result<(Self, ManagedObjects)> {
        // Create logger
        let logger = log_controller.logger("graphics");

        // Create DXGI info queue
        #[cfg(debug_assertions)]
        let dxgi_info_queue = DXGIInfoQueue::new(logger.clone())?;

        // Select adapter
        let mut selected_adapter = None;
        let mut adapters = Adapter::enumerate()?;
        if let Some(adapter_name) = settings.adapter.as_ref() {
            for (i, adapter) in adapters.iter().enumerate() {
                if adapter.name().starts_with(adapter_name) {
                    selected_adapter = Some(i);
                    info!(logger, "Found selected adapter!");
                    break;
                }
            }

            if selected_adapter.is_none() {
                warning!(
                    logger,
                    "Unable to find adapter named like \"{}\"",
                    adapter_name
                );

                for adapter in &adapters {
                    info!(
                        logger,
                        "Available adapter: \"{}\" ({} MB)",
                        adapter.name(),
                        adapter.video_memory() / 1000 / 1000
                    )
                }
            }
        }

        let mut adapter = match selected_adapter {
            Some(adapter) => adapters.swap_remove(adapter),
            None => {
                if adapters.len() == 0 {
                    return Err(Error::new("no valid adapters available"));
                }
                info!(logger, "Using default adapter");
                adapters.swap_remove(0)
            }
        };
        info!(
            logger,
            "Selected \"{}\" as the adapter ({} MB)",
            adapter.name(),
            adapter.video_memory() / 1000 / 1000
        );

        // Get size
        let size = message_thread.window_size();
        let width = size.x;
        let height = size.y;

        // Prepare swapchain description
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            buffer_desc: DXGI_MODE_DESC {
                width,
                height,
                format: SWAPCHAIN_FORMAT,
                ..Default::default()
            },
            buffer_usage: DXGI_USAGE_RENDER_TARGET_OUTPUT as _,
            buffer_count: BUFFER_COUNT,
            output_window: window,
            windowed: TRUE,
            swap_effect: DXGI_SWAP_EFFECT::FlipDiscard,
            flags: SWAP_CHAIN_FLAGS,
            ..Default::default()
        };

        // Create device, device context, and swapchain
        let mut device = null_mut();
        let mut device_context = null_mut();
        let mut swap_chain = null_mut();
        try_hresult!(D3D11CreateDeviceAndSwapChain(
            adapter.handle() as *mut _ as _,
            D3D_DRIVER_TYPE::Unknown,
            null_mut(),
            DEVICE_FLAGS,
            FEATURE_LEVELS.as_ptr(),
            2,
            D3D11_SDK_VERSION,
            &swap_chain_desc,
            &mut swap_chain,
            &mut device,
            null_mut(),
            &mut device_context
        ))
        .map_err(|os| Error::new_inner("unable to start D3D11", os))?;

        // Convert raw pointers into `ComPtr`s
        let mut device = ComPtr::new(device);
        let device_context = ComPtr::new(device_context);
        let swapchain = ComPtr::new(swap_chain);

        // Create info queue
        #[cfg(debug_assertions)]
        let d3d11_info_queue = D3D11InfoQueue::new(&mut device, logger.clone())?;

        // Create rasterizer state
        let rasterizer_desc = D3D11_RASTERIZER_DESC {
            front_counter_clockwise: TRUE,
            ..Default::default()
        };
        let rasterizer_state = ComPtr::new_in(|rasterizer_state| {
            try_hresult!(device.create_rasterizer_state(&rasterizer_desc, rasterizer_state))
        })
        .map_err(|error| Error::new_inner("unable to create rasterizer state", error))?;

        // Create blend state
        let blend_state_desc = D3D11_BLEND_DESC::default();
        let blend_state = ComPtr::new_in(|blend_state| {
            try_hresult!(device.create_blend_state(&blend_state_desc, blend_state))
        })
        .map_err(|error| Error::new_inner("unable to create blend state", error))?;

        // Create depth stencil state
        let depth_stencil_desc = D3D11_DEPTH_STENCIL_DESC::default();
        let depth_stencil_state = ComPtr::new_in(|depth_stencil_state| {
            try_hresult!(
                device.create_depth_stencil_state(&depth_stencil_desc, depth_stencil_state)
            )
        })
        .map_err(|error| Error::new_inner("unable to create depth stencil state", error))?;

        // Create managed objects
        let mut managed_objects = ManagedObjects::new(&device)?;

        // Create post processing objects
        let post_processing = PostProcessing::new(settings.anti_aliasing, &device)?;

        // Create render context and graphics context
        let mut graphics_context = GraphicsContext {
            logger,
            vsync: settings.vsync,
            display_mode: settings.display_mode,
            size: Vector2u::new(width, height),
            post_processing,
            render_scale: settings.render_scale,
            swapchain_objects: None,
            swapchain,
            depth_stencil_state,
            rasterizer_state,
            blend_state,
            device_context,
            device,
            message_thread,
            #[cfg(debug_assertions)]
            d3d11_info_queue,
            #[cfg(debug_assertions)]
            dxgi_info_queue,
        };

        // Force a resize
        graphics_context
            .force_resize(&mut managed_objects, graphics_context.size)
            .map_err(|error| error)?;

        Ok((graphics_context, managed_objects))
    }
}
