use crate::scene::Scene;
use crate::color::Color;
use crate::light::Light;
use crate::intersect::Intersect;  // Importar Intersect
use crate::sphere::Sphere;  // Importar Sphere
use crate::ray_intersect::RayIntersect;  // Importar el trait RayIntersect
use nalgebra_glm::Vec3;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Sphere],
) -> f32 {
    let bias = 0.001; // Pequeño desplazamiento para evitar acné
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * bias;
    let mut shadow_intensity = 0.9;

    for object in objects {
        if let Some(shadow_intersect) = object.ray_intersect(&shadow_ray_origin, &light_dir) {
            let distance_to_light = (light.position - intersect.point).magnitude();
            let distance_to_object = shadow_intersect.distance;
            if distance_to_object < distance_to_light {
                shadow_intensity = distance_to_object / distance_to_light;  // Sombras difusas según la distancia
                break;
            }
        }
    }

    shadow_intensity
}

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3, light: &Light) -> Color {
    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        // Calcular la dirección de la luz
        let light_dir = (light.position - intersection.point).normalize();

        // Calcular la intensidad de la sombra
        let shadow_intensity = cast_shadow(&intersection, light, &scene.spheres);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Calcular la intensidad difusa
        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse = intersection.material.diffuse * intersection.material.albedo[0] * diffuse_intensity * light_intensity;

        // Calcular la dirección de vista
        let view_dir = (ray_origin - intersection.point).normalize();

        // Calcular la dirección reflejada
        let reflect_dir = reflect(&-light_dir, &intersection.normal);

        // Calcular la intensidad especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersection.material.specular);
        let specular = light.color * intersection.material.albedo[1] * specular_intensity * light_intensity;

        // Sumar difusa y especular
        return diffuse + specular;
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0)
}
