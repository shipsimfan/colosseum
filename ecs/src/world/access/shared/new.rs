use crate::SharedWorldAccess;

impl SharedWorldAccess {
    /// Create a new instance of [`SharedWorldAccess`]
    pub(in crate::world::access) fn new() -> SharedWorldAccess {
        SharedWorldAccess {
            read_resources: Vec::new(),
            write_resources: Vec::new(),
        }
    }
}
