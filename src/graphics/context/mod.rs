use crate::{MessageThread, graphics::DisplayMode, logging::Logger, math::Vector2u};
#[cfg(debug_assertions)]
use d3d11_info_queue::D3D11InfoQueue;
#[cfg(debug_assertions)]
use dxgi_info_queue::DXGIInfoQueue;
use managed_objects::ManagedGraphicsObjects;
use std::rc::Rc;
use swapchain_objects::SwapchainObjects;
use win32::{
    ComPtr, UINT,
    d3d11::{
        ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext,
        ID3D11RasterizerState,
    },
    dxgi::{DXGI_FORMAT, DXGI_SWAP_CHAIN_FLAG, IDXGISwapChain},
};

#[cfg(debug_assertions)]
mod d3d11_info_queue;
#[cfg(debug_assertions)]
mod dxgi_info_queue;
mod managed_objects;
mod swapchain_objects;

mod create;
mod get;
mod log_debug_messages;
mod new;
mod render;
mod resize;
mod set;

pub(in crate::graphics) use managed_objects::Lights;

/// The context for creating graphics objects and rendering using them
pub struct GraphicsContext {
    /// The logger for graphics events
    logger: Logger,

    /// Should presents be synchronized with the vertical blank?
    vsync: bool,

    /// The current display mode of the window
    display_mode: DisplayMode,

    /// The current size of the swapchain
    size: Vector2u,

    /// The objects which the engine manages but are created by the game
    managed_objects: ManagedGraphicsObjects,

    /// The objects directly associated with the swapchain
    swapchain_objects: Option<SwapchainObjects>,

    /// The swap chain to render onto
    swapchain: ComPtr<IDXGISwapChain>,

    /// The state describing how the depth stecil view should work
    depth_stencil_state: ComPtr<ID3D11DepthStencilState>,

    /// The state to use when blending different meshes
    blend_state: ComPtr<ID3D11BlendState>,

    /// The state to use for the rasterizer
    rasterizer_state: ComPtr<ID3D11RasterizerState>,

    /// The device context for issuing rendering commands
    device_context: ComPtr<ID3D11DeviceContext>,

    /// The info queue producing debug messages from Direct3D 11
    #[cfg(debug_assertions)]
    d3d11_info_queue: D3D11InfoQueue,

    /// The device for creating render objects
    device: ComPtr<ID3D11Device>,

    /// The thread controlling the window
    message_thread: Rc<MessageThread>,

    /// The info queue producing debug messages from DXGI
    #[cfg(debug_assertions)]
    dxgi_info_queue: DXGIInfoQueue,
}

pub(in crate::graphics) const RENDER_TARGET_FORMAT: DXGI_FORMAT = DXGI_FORMAT::B8G8R8A8UNorm;
const DEPTH_FORMAT: DXGI_FORMAT = DXGI_FORMAT::D32Float;
const BUFFER_COUNT: UINT = 3;
const SWAP_CHAIN_FLAGS: UINT = DXGI_SWAP_CHAIN_FLAG::AllowTearing as _;
