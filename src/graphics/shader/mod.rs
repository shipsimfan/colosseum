use std::rc::Rc;

mod inner;
mod source;

mod default;
mod deref;
mod new;

pub use inner::ShaderInner;
pub use source::ShaderSource;

/// A shader program which can be used to render
#[derive(Clone)]
pub struct Shader {
    /// The reference to the shader itself
    shader: Rc<ShaderInner>,
}
