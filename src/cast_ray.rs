use crate::scene::Scene;
use crate::color::Color;
use nalgebra_glm::{dot, Vec3};

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3) -> Color {
    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        // Calcular la dirección de la luz y la intensidad
        let light_direction = (scene.light_position - intersection.point).normalize();
        let light_intensity = dot(&intersection.normal, &light_direction).max(0.0);

        // Aplicar la intensidad de luz al color del material
        let color = intersection.material.diffuse * light_intensity;
        return color;
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0)
}
