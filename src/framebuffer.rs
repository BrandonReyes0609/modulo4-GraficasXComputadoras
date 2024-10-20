use nalgebra_glm::Vec3;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec3>, // Almacena los colores de los píxeles
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![Vec3::new(0.0, 0.0, 0.0); width * height], // Inicializa con color negro
        }
    }

    pub fn set_current_color(&mut self, x: usize, y: usize, color: Vec3) {
        self.buffer[y * self.width + x] = color;
    }

    pub fn point(&mut self, x: usize, y: usize, color: Vec3) {
        self.set_current_color(x, y, color);
    }
}
