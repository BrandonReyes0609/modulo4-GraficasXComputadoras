mod framebuffer;
mod cast_ray;
mod render;
mod ray_intersect;
mod sphere;

use crate::framebuffer::Framebuffer;
use crate::render::render;
use crate::sphere::Sphere;
use nalgebra_glm::Vec3;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Crear una esfera en el centro de la escena
    let objects = vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -5.0), // Centro en (0, 0, -5)
            radius: 1.0,                        // Radio de 1 unidad
        },
    ];

    render(&mut framebuffer, &objects);

    // Aquí puedes mostrar o guardar el framebuffer
    println!("Renderizado completado");
}
