use graphics::{ConstantBuffer, Input, Matrix};

pub struct Window<I: Input> {
    window: Box<graphics::Window<I>>,

    camera_matrix: Matrix,

    camera_constant_buffer: ConstantBuffer<Matrix, 0>,
    object_constant_buffer: ConstantBuffer<Matrix, 1>,
}

impl<I: Input> Window<I> {
    pub fn new<S: AsRef<str>>(title: S, width: usize, height: usize) -> Self {
        let mut window = graphics::Window::new(title.as_ref(), width, height).unwrap();

        let identity = Matrix::identity();

        let camera_constant_buffer = ConstantBuffer::new(Some(identity), &mut window).unwrap();
        let object_constant_buffer = ConstantBuffer::new(Some(identity), &mut window).unwrap();

        Window {
            window,
            camera_matrix: identity,
            camera_constant_buffer,
            object_constant_buffer,
        }
    }

    pub fn width(&self) -> f32 {
        self.window.get_width() as f32
    }

    pub fn height(&self) -> f32 {
        self.window.get_height() as f32
    }

    pub fn current_camera_matrix(&self) -> &Matrix {
        &self.camera_matrix
    }

    pub fn input(&self) -> &I {
        self.window.input()
    }

    pub fn set_camera_matrix(&mut self, matrix: Matrix) {
        self.camera_matrix = matrix;

        self.camera_constant_buffer
            .set(matrix, &mut self.window)
            .unwrap();
    }

    pub fn set_object_matrix(&mut self, matrix: Matrix) {
        self.object_constant_buffer
            .set(matrix, &mut self.window)
            .unwrap();
    }

    pub fn update_camera_buffer(&mut self) {
        self.camera_constant_buffer.set_active(&mut self.window);
        self.object_constant_buffer.set_active(&mut self.window);
    }

    pub fn inner(&mut self) -> &mut Box<graphics::Window<I>> {
        &mut self.window
    }
}
