/// Convert a reference to a type into a byte slice
pub(in crate::render) unsafe fn as_bytes<T>(data: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(std::ptr::from_ref(data).cast(), std::mem::size_of::<T>()) }
}
