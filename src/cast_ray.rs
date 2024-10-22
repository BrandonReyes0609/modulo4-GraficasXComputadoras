use crate::scene::Scene;
use crate::color::Color;
use crate::light::Light; // Asegúrate de importar la estructura Light
use nalgebra_glm::Vec3;

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3, light: &Light) -> Color {
    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        // Calcular la dirección de la luz
        let light_dir = (light.position - intersection.point).normalize();

        // Calcular la intensidad difusa
        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0);
        let diffuse = intersection.material.diffuse * diffuse_intensity * light.intensity;

        // Devolver el color con la luz difusa aplicada
        return diffuse;
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0)
}
