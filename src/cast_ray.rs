use nalgebra_glm::Vec3;
use crate::sphere::Sphere;
use crate::intersect::Intersect;
use crate::color::Color;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY; // Z-buffer para guardar la distancia más cercana

    for object in objects.iter() {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance; // Guardar la distancia más cercana
            closest_intersect = tmp;
        }
    }

    if !closest_intersect.is_intersecting {
        // Si no hay intersección, devolver un color de fondo (skybox)
        return Color::new(4, 12, 36); // Color del "cielo"
    }

    // Si hay intersección, devolver el color difuso del material del objeto
    closest_intersect.material.diffuse
}
