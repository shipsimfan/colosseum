use crate::graphics::lights::SpotLight;
use std::rc::Rc;

impl Drop for SpotLight {
    fn drop(&mut self) {
        // Is this the last reference to the spot light, other than from the list?
        if Rc::strong_count(&self.spot_light) == 2 {
            // If so, remove it from the list
            let mut spot_light_list = self.spot_light_list.borrow_mut();
            let mut index = None;
            for (i, spot_light) in spot_light_list.0.iter().enumerate() {
                if Rc::ptr_eq(spot_light, &self.spot_light) {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                spot_light_list.0.remove(i);
                spot_light_list.1 = true;
            }
        }
    }
}
