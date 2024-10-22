use crate::scene::Scene;
use crate::color::Color;
use crate::light::Light;
use nalgebra_glm::Vec3;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3, light: &Light) -> Color {
    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        // Calcular la dirección de la luz
        let light_dir = (light.position - intersection.point).normalize();

        // Calcular la intensidad difusa
        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse = intersection.material.diffuse * intersection.material.albedo[0] * diffuse_intensity * light.intensity;

        // Calcular la dirección de vista
        let view_dir = (ray_origin - intersection.point).normalize();

        // Calcular la dirección reflejada
        let reflect_dir = reflect(&-light_dir, &intersection.normal);

        // Calcular la intensidad especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersection.material.specular);
        let specular = light.color * intersection.material.albedo[1] * specular_intensity * light.intensity;

        // Sumar difusa y especular
        return diffuse + specular;
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0)
}
