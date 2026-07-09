use crate::update::SystemId;

impl std::fmt::Display for SystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.phase, self.id)
    }
}
