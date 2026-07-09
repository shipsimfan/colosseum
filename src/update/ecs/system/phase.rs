/// A phase that a system can be in, which determines when it is executed in the update loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemPhase {
    /// The system is executed before the main update loop
    PreUpdate,

    /// The system can be executed at any time during the main update loop
    AdHoc,

    /// The system is executed after the main update loop
    PostUpdate,
}

impl std::fmt::Display for SystemPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemPhase::PreUpdate => "PreUpdate",
            SystemPhase::AdHoc => "AdHoc",
            SystemPhase::PostUpdate => "PostUpdate",
        }
        .fmt(f)
    }
}
