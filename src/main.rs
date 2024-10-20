mod framebuffer;
mod cast_ray;
mod render;
mod sphere;
mod material;
mod intersect;
mod color;

use crate::framebuffer::Framebuffer;
use crate::render::render;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::color::Color;
use nalgebra_glm::Vec3;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    let red_material = Material::new(Color::new(255, 0, 0));
    let green_material = Material::new(Color::new(0, 255, 0));

    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, red_material),
        Sphere::new(Vec3::new(2.0, 0.0, -5.0), 1.0, green_material),
    ];

    render(&mut framebuffer, &objects[..]);

    // Escribir el framebuffer en un archivo PPM
    let mut file = File::create("output.ppm")?;
    write_ppm(&mut file, &framebuffer)?;

    println!("Renderizado completado y guardado como output.ppm");

    Ok(())
}

fn write_ppm(file: &mut File, framebuffer: &Framebuffer) -> io::Result<()> {
    // Cabecera del archivo PPM
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", framebuffer.width, framebuffer.height)?;
    writeln!(file, "255")?;

    // Escribir los datos de color de cada píxel
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let pixel = framebuffer.get_pixel(x, y);
            let r = (pixel.x * 255.0) as u32;
            let g = (pixel.y * 255.0) as u32;
            let b = (pixel.z * 255.0) as u32;
            writeln!(file, "{} {} {}", r, g, b)?;
        }
    }

    Ok(())
}
