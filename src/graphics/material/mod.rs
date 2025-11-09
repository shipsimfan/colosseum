use std::{cell::RefCell, rc::Rc};

mod borrow;
mod drop;
mod inner;
mod new;

pub use inner::MaterialInner;

/// A item which controls the way meshes are rendered
#[derive(Clone)]
pub struct Material {
    /// The list of current materials that contains this materials
    material_list: Rc<RefCell<Vec<Rc<RefCell<MaterialInner>>>>>,

    /// The reference to the material itself
    material: Rc<RefCell<MaterialInner>>,
}
