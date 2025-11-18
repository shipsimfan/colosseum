use crate::{Error, Result, graphics::context::DXGIInfoQueue, logging::Logger};
use win32::{
    ComInterface, ComPtr, UINT64,
    dxgi1_3::DXGIGetDebugInterface1,
    dxgidebug::{
        DXGI_DEBUG_ALL, DXGI_DEBUG_D3D11, DXGI_INFO_QUEUE_FILTER, DXGI_INFO_QUEUE_FILTER_DESC,
        DXGI_INFO_QUEUE_MESSAGE_SEVERITY, IDXGIInfoQueue,
    },
    try_hresult,
};

impl DXGIInfoQueue {
    /// Create a new [`InfoQueue`]
    pub fn new(logger: Logger) -> Result<Self> {
        let mut handle = ComPtr::<IDXGIInfoQueue>::new_in(|info_queue| {
            try_hresult!(DXGIGetDebugInterface1(
                0,
                &IDXGIInfoQueue::IID,
                info_queue.cast()
            ))
        })
        .map_err(|error| Error::new_inner("unable to get info queue", error))?;

        // Allow all messages with no limit
        try_hresult!(handle.set_message_count_limit(DXGI_DEBUG_D3D11, UINT64::MAX))
            .map_err(|error| Error::new_inner("unable to clear info queue message limit", error))?;
        handle.clear_retrieval_filter(DXGI_DEBUG_ALL);
        handle.clear_storage_filter(DXGI_DEBUG_ALL);

        // Block "INFO" messages
        handle.add_storage_filter_entries(
            DXGI_DEBUG_ALL,
            &mut DXGI_INFO_QUEUE_FILTER {
                deny_list: DXGI_INFO_QUEUE_FILTER_DESC {
                    num_severities: 1,
                    severity_list: &mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY::Info,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Ok(DXGIInfoQueue { handle, logger })
    }
}
