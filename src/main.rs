mod framebuffer;
mod cast_ray;
mod render;

use crate::framebuffer::Framebuffer;
use crate::cast_ray::Object;
use crate::render::render;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    let objects: Vec<Object> = vec![]; // Placeholder para objetos en la escena
    render(&mut framebuffer, &objects);

    // En una implementación real, aquí se podría mostrar o guardar el framebuffer
    println!("Renderizado completado");
}
