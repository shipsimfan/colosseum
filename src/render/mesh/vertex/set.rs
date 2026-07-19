use crate::render::Vertex;
use alexandria::math::{Color3f, Linear, Vector3f};

impl Vertex {
    /// Set the position of the vertex
    pub fn set_position<Position: Into<Vector3f>>(&mut self, position: Position) {
        self.position = position.into();
    }

    /// Set the x coordinate of the vertex
    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    /// Set the y coordinate of the vertex
    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    /// Set the z coordinate of the vertex
    pub fn set_z(&mut self, z: f32) {
        self.position.z = z;
    }

    /// Set the color of the vertex
    pub fn set_color<Color: Into<Color3f<Linear>>>(&mut self, color: Color) {
        self.color = color.into();
    }

    /// Set the red component of the vertex color
    pub fn set_r(&mut self, r: f32) {
        self.color.r = r;
    }

    /// Set the green component of the vertex color
    pub fn set_g(&mut self, g: f32) {
        self.color.g = g;
    }

    /// Set the blue component of the vertex color
    pub fn set_b(&mut self, b: f32) {
        self.color.b = b;
    }
}
