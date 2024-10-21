use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

impl Material {
    pub fn new(diffuse: Color) -> Self {
        Self { diffuse }
    }

    pub fn black() -> Self {
        Self {
            diffuse: Color::black(),
        }
    }
}
