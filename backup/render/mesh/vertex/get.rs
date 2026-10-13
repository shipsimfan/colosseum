use crate::render::Vertex;
use alexandria::math::{Color3f, Linear, Vector3f};

impl Vertex {
    /// Get the position of the vertex
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the x coordinate of the vertex
    pub fn x(&self) -> f32 {
        self.position.x
    }

    /// Get the y coordinate of the vertex
    pub fn y(&self) -> f32 {
        self.position.y
    }

    /// Get the z coordinate of the vertex
    pub fn z(&self) -> f32 {
        self.position.z
    }

    /// Get the color of the vertex
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }

    /// Get the red component of the vertex color
    pub fn r(&self) -> f32 {
        self.color.r
    }

    /// Get the green component of the vertex color
    pub fn g(&self) -> f32 {
        self.color.g
    }

    /// Get the blue component of the vertex color
    pub fn b(&self) -> f32 {
        self.color.b
    }
}
