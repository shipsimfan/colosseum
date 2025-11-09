use crate::graphics::{Material, MaterialInner};
use std::{cell::RefCell, rc::Rc};

impl Material {
    /// Create a new [`Material`]
    pub(in crate::graphics) fn new(
        material_list: Rc<RefCell<Vec<Rc<RefCell<MaterialInner>>>>>,
        material: Rc<RefCell<MaterialInner>>,
    ) -> Self {
        Material {
            material_list,
            material,
        }
    }
}
