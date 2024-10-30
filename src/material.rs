use crate::color::Color;
use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub texture: Option<DynamicImage>,
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 4], refractive_index: f32, texture: Option<DynamicImage>) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0.0, 0.0, 0.0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            texture: None,
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(ref texture) = self.texture {
            let x = (u * (texture.width() as f32 - 1.0)).round() as u32;
            let y = (v * (texture.height() as f32 - 1.0)).round() as u32;
            let tex_color = texture.get_pixel(x, y);
            Color::new(tex_color[0] as f32 / 255.0, tex_color[1] as f32 / 255.0, tex_color[2] as f32 / 255.0)
        } else {
            self.diffuse
        }
    }
}
