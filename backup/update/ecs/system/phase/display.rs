use crate::update::SystemPhase;

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
