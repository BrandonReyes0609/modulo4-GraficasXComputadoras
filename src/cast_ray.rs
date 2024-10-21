// Elimina esta línea si no la necesitas
// use crate::ray_intersect::RayIntersect;
use crate::scene::Scene;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3) -> Color {
    let intersection = scene.ray_intersect(ray_origin, ray_direction);

    if intersection.is_intersecting {
        let light_direction = (scene.light_position - intersection.point).normalize();
        let light_intensity = nalgebra_glm::dot(&intersection.normal, &light_direction).max(0.0);
        let color = Color::new(255.0, 255.0, 255.0) * light_intensity;
        return color;
    }

    Color::new(4.0, 12.0, 36.0) // Color del cielo
}
