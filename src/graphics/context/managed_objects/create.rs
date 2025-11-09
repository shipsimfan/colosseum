use crate::{
    Result,
    graphics::{Camera, CameraInner, CameraProjection, context::ManagedGraphicsObjects},
    math::Vector2u,
};
use std::{cell::RefCell, rc::Rc};
use win32::d3d11::ID3D11Device;

impl ManagedGraphicsObjects {
    /// Creates a new [`Camera`]
    pub fn create_camera(
        &self,
        projection: CameraProjection,
        screen_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Camera> {
        let camera = Rc::new(RefCell::new(CameraInner::new(
            projection,
            screen_size,
            device,
        )?));

        self.cameras.borrow_mut().push(camera.clone());

        Ok(Camera::new(self.cameras.clone(), camera))
    }
}
