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
use crate::color::Color; // Importar correctamente el tipo Color
use nalgebra_glm::Vec3;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Utilizamos Color en lugar de Vec3 para definir los colores
    let red_material = Material::new(Color::new(255, 0, 0));
    let green_material = Material::new(Color::new(0, 255, 0));

    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, red_material),
        Sphere::new(Vec3::new(2.0, 0.0, -5.0), 1.0, green_material),
    ];

    render(&mut framebuffer, &objects[..]); // Pasamos el slice en lugar del `Vec`

    println!("Renderizado completado");
}
