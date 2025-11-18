use crate::{Error, Result, graphics::context::D3D11InfoQueue, logging::Logger};
use win32::{
    ComPtr,
    d3d11::ID3D11Device,
    d3d11sdklayers::{
        D3D11_INFO_QUEUE_FILTER, D3D11_INFO_QUEUE_FILTER_DESC, D3D11_MESSAGE_SEVERITY,
        ID3D11InfoQueue,
    },
    try_hresult,
};

impl D3D11InfoQueue {
    /// Create a new [`InfoQueue`]
    pub fn new(device: &mut ComPtr<ID3D11Device>, logger: Logger) -> Result<Self> {
        let mut handle: ComPtr<ID3D11InfoQueue> = device
            .query_interface()
            .map_err(|error| Error::new_inner("unable to get device info queue", error))?;

        // Allow all messages with no limit
        try_hresult!(handle.set_message_count_limit(-1 as _))
            .map_err(|error| Error::new_inner("unable to clear info queue message limit", error))?;
        handle.clear_retrieval_filter();
        handle.clear_storage_filter();

        // Block "INFO" messages
        handle.add_storage_filter_entries(&mut D3D11_INFO_QUEUE_FILTER {
            deny_list: D3D11_INFO_QUEUE_FILTER_DESC {
                num_severities: 1,
                severity_list: &mut D3D11_MESSAGE_SEVERITY::Info,
                ..Default::default()
            },
            ..Default::default()
        });

        Ok(D3D11InfoQueue { handle, logger })
    }
}
