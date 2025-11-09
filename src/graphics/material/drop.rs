use crate::graphics::Material;
use std::rc::Rc;

impl Drop for Material {
    fn drop(&mut self) {
        // Is this the last reference to the material, other than from the list?
        if Rc::strong_count(&self.material) == 2 {
            // If so, remove it from the list
            let mut material_list = self.material_list.borrow_mut();
            let mut index = None;
            for (i, material) in material_list.iter().enumerate() {
                if Rc::ptr_eq(material, &self.material) {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                material_list.remove(i);
            }
        }
    }
}
