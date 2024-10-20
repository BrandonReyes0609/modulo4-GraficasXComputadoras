extern crate image;
extern crate minifb;

use image::{RgbImage, Rgb};
use minifb::{Key, Window, WindowOptions};
use std::f32;

// Definición del vector 3D
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

// Definición de Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }
}

// Estructura del Material
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

// Estructura de la Intersección
#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(distance: f32, material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: f32::INFINITY,
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0),
            },
        }
    }
}

// Definición de la Esfera
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin.sub(&self.center);
        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let t = if t1 < t2 { t1 } else { t2 };

            if t > 0.0 {
                return Intersect::new(t, self.material);
            }
        }

        Intersect::empty()
    }
}

// Framebuffer para almacenar y manipular los píxeles de la pantalla
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub image: RgbImage,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            image: RgbImage::new(width as u32, height as u32),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if x < self.width && y < self.height {
            self.image.put_pixel(x as u32, y as u32, Rgb([
                (color.x.min(1.0).max(0.0) * 255.0) as u8,
                (color.y.min(1.0).max(0.0) * 255.0) as u8,
                (color.z.min(1.0).max(0.0) * 255.0) as u8,
            ]));
        }
    }

    pub fn save_as_image(&self, filename: &str) {
        self.image.save(filename).expect("Failed to save image");
    }

    // Convertir la imagen en un buffer de u32 para minifb
    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.image.get_pixel(x as u32, y as u32);
                let color = Color::new(pixel[0], pixel[1], pixel[2]);
                buffer.push(color.to_u32());
            }
        }
        buffer
    }
}

// Función para lanzar rayos y determinar el color de cada píxel
pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest_intersection = Intersect::empty();

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < closest_intersection.distance {
            closest_intersection = tmp;
        }
    }

    if closest_intersection.is_intersecting {
        closest_intersection.material.diffuse
    } else {
        Color::new(4, 12, 36)
    }
}

// Función principal de renderizado
pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            framebuffer.set_pixel(x, y, pixel_color.to_vec3());
        }
    }
}

// Función principal
fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    let ivory = Material {
        diffuse: Color::new(200, 200, 200),
    };
    let rubber = Material {
        diffuse: Color::new(80, 20, 20),
    };

    let objects = vec![
        Sphere {
            center: Vec3::new(1.0, 0.0, -4.0),
            radius: 1.0,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(2.0, 0.0, -5.0),
            radius: 1.0,
            material: rubber,
        },
    ];

    render(&mut framebuffer, &objects);
    framebuffer.save_as_image("output.png");
    println!("Renderizado completado. Imagen guardada como 'output.png'.");

    // Mostrar la imagen en una ventana usando minifb
    let mut window = Window::new(
        "Raytraced Image",
        width,
        height,
        WindowOptions::default(),
    ).expect("Unable to open window");

    let buffer = framebuffer.to_u32_buffer();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
