use crate::{Error, Result};
use win32::{ExpandEnvironmentStrings, try_get_last_error};

const BUFFER_SIZE: usize = 4096;

/// Expand any environment variables in `string`
pub(crate) fn expand_environment_string(string: &str) -> Result<String> {
    let mut utf16_string: Vec<_> = string.encode_utf16().collect();
    utf16_string.push(0);

    let mut output_buffer = Vec::with_capacity(BUFFER_SIZE);
    let path_length = try_get_last_error!(ExpandEnvironmentStrings(
        utf16_string.as_ptr(),
        output_buffer.as_mut_ptr(),
        BUFFER_SIZE as _
    ))
    .map_err(|error| Error::new_inner(format!("unable to expand \"{}\"", string), error))?
        - 1;
    unsafe { output_buffer.set_len(path_length as _) };

    Ok(String::from_utf16_lossy(&output_buffer))
}
