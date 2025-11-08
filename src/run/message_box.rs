use crate::{Error, Result};
use std::ptr::null_mut;
use win32::{MB_ICONERROR, MB_OK, MessageBox, try_get_last_error};

/// Display a message box with an "ok" option
pub(in crate::run) fn message_box(title: &str, content: &str) -> Result<()> {
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);

    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);

    try_get_last_error!(MessageBox(
        null_mut(),
        content.as_ptr(),
        title.as_ptr(),
        MB_OK | MB_ICONERROR
    ))
    .map_err(|error| Error::new_inner("unable to display message box", error))
    .map(|_| ())
}
