use nalgebra_glm::{normalize, Vec3};
use crate::framebuffer::Framebuffer;
use crate::cast_ray::cast_ray;
use crate::sphere::Sphere;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Mapea la coordenada del píxel al espacio de pantalla [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Ajusta por el aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calcula la dirección del rayo para este píxel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Lanza el rayo y obtiene el objeto intersectado
            let intersect = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Determina el color a dibujar basado en el material del objeto intersectado
            let color = if intersect.is_intersecting {
                let material = intersect.material;
                Vec3::new(material.diffuse.r as f32 / 255.0, material.diffuse.g as f32 / 255.0, material.diffuse.b as f32 / 255.0)
            } else {
                Vec3::new(0.0, 0.0, 0.0) // Color negro si no hay intersección
            };

            // Dibuja el píxel en pantalla con el color determinado
            framebuffer.point(x, y, color);
        }
    }
}
