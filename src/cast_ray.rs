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

pub fn cast_ray(scene: &Scene, ray_origin: &Vec3, ray_direction: &Vec3, light: &Light, depth: u32) -> Color {
    if depth > 3 {
        return Color::new(0.2, 0.7, 1.0); // Limitar la profundidad de recursión
    }

    if let Some(intersection) = scene.ray_intersect(ray_origin, ray_direction) {
        let light_dir = (light.position - intersection.point).normalize();

        let shadow_intensity = cast_shadow(&intersection, light, &scene.spheres);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        let diffuse_intensity = intersection.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse = intersection.material.diffuse * intersection.material.albedo[0] * diffuse_intensity * light_intensity;

        let view_dir = (ray_origin - intersection.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersection.normal);
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersection.material.specular);
        let specular = light.color * intersection.material.albedo[1] * specular_intensity * light_intensity;

        // Calcular el color reflejado
        let mut reflect_color = Color::black();
        let reflectivity = intersection.material.albedo.get(2).copied().unwrap_or(0.0);
        if reflectivity > 0.0 {
            let reflect_dir = reflect(&-ray_direction, &intersection.normal).normalize();
            let reflect_origin = intersection.point + intersection.normal * 0.001; // Offset para evitar acné
            reflect_color = cast_ray(scene, &reflect_origin, &reflect_dir, light, depth + 1);
        }

        // Calcular el color refractado
        let mut refract_color = Color::black();
        let transparency = intersection.material.albedo[3];
        if transparency > 0.0 {
            let refract_dir = refract(ray_direction, &intersection.normal, intersection.material.refractive_index);
            let refract_origin = intersection.point + intersection.normal * 0.001; // Offset para evitar acné
            refract_color = cast_ray(scene, &refract_origin, &refract_dir, light, depth + 1);
        }

        // Combinar colores difuso, especular, reflejado y refractado
        return (diffuse + specular) * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency);
    }

    // Color de fondo (cielo)
    Color::new(0.2, 0.7, 1.0)
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // El rayo está entrando en el objeto
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // El rayo está saliendo del objeto
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        // Reflexión interna total
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}