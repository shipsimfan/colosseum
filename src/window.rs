use crate::{shader::ObjectBuffer, Mesh, Sprite, Texture};
use alexandria::{ConstantBuffer, Input, Matrix, Vector4};

pub struct Window<I: Input> {
    window: Box<alexandria::Window<I>>,

    camera_matrix: Matrix,

    camera_constant_buffer: ConstantBuffer<Matrix>,
    object_constant_buffer: ConstantBuffer<ObjectBuffer>,

    fixed_update_time: Option<f32>,

    default_texture: Option<Texture>,
    default_sprite_mesh: Option<Mesh>,
}

impl<I: Input> Window<I> {
    pub(crate) fn new<S: AsRef<str>>(
        title: S,
        width: usize,
        height: usize,
        fixed_update_time: Option<f32>,
        debug_logging: bool,
    ) -> Self {
        let mut window = Box::new(
            alexandria::Window::<I>::new(title.as_ref(), width, height, debug_logging).unwrap(),
        );

        let identity = Matrix::identity();

        let camera_constant_buffer = ConstantBuffer::new(identity, 0, &mut window).unwrap();
        let object_constant_buffer =
            ConstantBuffer::new(ObjectBuffer::new(identity, Vector4::ONE), 1, &mut window).unwrap();

        Window {
            window,
            camera_matrix: identity,
            camera_constant_buffer,
            object_constant_buffer,
            fixed_update_time,
            default_texture: None,
            default_sprite_mesh: None,
        }
    }

    pub fn width(&self) -> f32 {
        self.window.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.window.height() as f32
    }

    pub fn aspect(&self) -> f32 {
        self.width() / self.height()
    }

    pub fn inv_aspect(&self) -> f32 {
        self.height() / self.width()
    }

    pub fn current_camera_matrix(&self) -> &Matrix {
        &self.camera_matrix
    }

    pub fn input(&self) -> &I {
        self.window.input()
    }

    pub fn fixed_update_time(&self) -> Option<f32> {
        self.fixed_update_time
    }

    pub fn default_texture(&self) -> &Texture {
        self.default_texture.as_ref().unwrap()
    }

    pub(super) fn post_init(&mut self) {
        // Initialize the default texture
        let mut image = ginger::Image::new(1, 1);
        image.set_pixel(0, 0, ginger::Pixel::new(1.0, 1.0, 1.0));

        self.default_texture = Some(Texture::new(image, alexandria::SampleType::Linear, self));

        // Initialize the sprite mesh
        self.default_sprite_mesh = Some(Sprite::create_default_sprite_mesh(self));
    }

    pub fn set_fixed_update_time(&mut self, delta_time: Option<f32>) {
        self.fixed_update_time = delta_time;
    }

    pub fn set_mouse_lock(&mut self, lock: bool) {
        self.window.input_mut().set_mouse_lock(lock);
    }

    pub fn set_camera_matrix(&mut self, matrix: Matrix) {
        self.camera_matrix = matrix;

        self.camera_constant_buffer.set_data(matrix).unwrap();
    }

    pub fn set_object_buffer(&mut self, matrix: Matrix, tint: Vector4) {
        self.object_constant_buffer
            .set_data(ObjectBuffer::new(matrix, tint))
            .unwrap();
    }

    pub fn update_camera_buffer(&mut self) {
        self.camera_constant_buffer.set_active();
        self.object_constant_buffer.set_active();
    }

    pub fn inner(&mut self) -> &mut Box<alexandria::Window<I>> {
        &mut self.window
    }

    pub(crate) fn render_default_sprite_mesh(&mut self) {
        self.default_sprite_mesh.as_mut().unwrap().render();
    }
}
