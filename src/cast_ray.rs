use crate::scene::Scene;
use crate::color::Color;
use crate::light::Light; // Asegúrate de importar la estructura Light
use nalgebra_glm::Vec3;

// Función para calcular la dirección reflejada
fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3, light: &Light) -> Color {
    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        // Calcular la dirección de la luz
        let light_dir = (light.position - intersection.point).normalize();

        // Calcular la intensidad difusa usando el modelo Lambertiano
        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0);
        let diffuse = intersection.material.diffuse * diffuse_intensity * light.intensity;

        // Calcular la dirección de vista (de la cámara al punto de intersección)
        let view_dir = (ray_origin - intersection.point).normalize();

        // Calcular la dirección reflejada
        let reflect_dir = reflect(&-light_dir, &intersection.normal);

        // Calcular la intensidad especular utilizando el modelo Phong
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersection.material.specular);
        let specular = light.color * specular_intensity * light.intensity;

        // Sumar difusa y especular para obtener el color final
        return diffuse + specular;
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0) // Un color de cielo por defecto si no hay intersección
}
