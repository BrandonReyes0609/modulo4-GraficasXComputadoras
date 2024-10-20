use nalgebra_glm::{normalize, Vec3};
use crate::framebuffer::Framebuffer;
use crate::cast_ray::cast_ray;
use crate::sphere::Sphere;
use crate::camera::Camera;  // Usar la cámara

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
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

            // Transformar la dirección del rayo con la cámara
            let transformed_ray = camera.basis_change(&ray_direction);

            // Lanza el rayo y obtiene el color correspondiente
            let pixel_color = cast_ray(&camera.eye, &transformed_ray, objects);

            // Dibuja el píxel en pantalla con el color devuelto
            framebuffer.point(x, y, Vec3::new(
                (pixel_color.r as f32) / 255.0,
                (pixel_color.g as f32) / 255.0,
                (pixel_color.b as f32) / 255.0,
            ));
        }
    }
}
