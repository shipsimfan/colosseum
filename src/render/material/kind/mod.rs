mod display;

/// How a material is rendered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialKind {
    /// The material is opaque and unlit
    ///
    /// This type of material is run in a forward pass without any lighting calculations
    UnlitOpaque,
}
