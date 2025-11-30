use crate::graphics::lights::DirectionalLight;
use std::rc::Rc;

impl Drop for DirectionalLight {
    fn drop(&mut self) {
        // Is this the last reference to the directional light, other than from the list?
        if Rc::strong_count(&self.directional_light) == 2 {
            // If so, remove it from the list
            let mut directional_light_list = self.directional_light_list.borrow_mut();
            let mut index = None;
            for (i, directional_light) in directional_light_list.0.iter().enumerate() {
                if Rc::ptr_eq(directional_light, &self.directional_light) {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                directional_light_list.0.remove(i);
                directional_light_list.1 = true;
            }
        }
    }
}
