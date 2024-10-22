use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,  // Coeficiente especular
}

impl Material {
    pub fn new(diffuse: Color, specular: f32) -> Self {
        Self {
            diffuse,
            specular,
        }
    }

    pub fn black() -> Self {
        Self {
            diffuse: Color::black(),
            specular: 0.0,
        }
    }
}
